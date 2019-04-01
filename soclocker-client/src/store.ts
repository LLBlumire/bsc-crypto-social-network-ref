import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";
import nacl from "tweetnacl";
import * as base64 from "@stablelib/base64";
import { AuthResponse } from "@/model.ts";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    username: new String(),
    secretKey: new Uint8Array(),
    loginAttemptFails: 0,
    isLoggedIn: false
  },
  mutations: {
    logout(state) {
      state.username = new String();
      state.secretKey = new Uint8Array();
      state.loginAttemptFails = 0;
      state.isLoggedIn = false;
    },
    login(
      state,
      {
        username,
        secretKey
      }: {
        username: string;
        secretKey: Uint8Array;
      }
    ) {
      state.username = username;
      state.secretKey = secretKey;
      state.loginAttemptFails = 0;
      state.isLoggedIn = true;
    },
    incrementLoginFails(state) {
      state.loginAttemptFails += 1;
    }
  },
  actions: {
    handleLogout({ commit }) {
      commit("logout");
    },
    handleLogin(
      { commit },
      {
        username,
        secretKeyB64
      }: {
        username: string;
        secretKeyB64: string;
      }
    ) {
      axios
        .get("/_/server_public_key")
        .then(spk => {
          let serverPublicKeyB64: string = spk.data;
          axios
            .get("/_/auth", { params: { username: username } })
            .then(auth => {
              let authResponseB64: AuthResponse = auth.data;
              let encryptedToken: Uint8Array = base64.decode(
                authResponseB64.encryptedToken
              );
              let nonce: Uint8Array = base64.decode(authResponseB64.nonce);
              let serverPublicKey: Uint8Array = base64.decode(
                serverPublicKeyB64
              );
              let localSecretKey: Uint8Array = base64.decode(secretKeyB64);
              let box: Uint8Array | null = nacl.box.open(
                encryptedToken,
                nonce,
                serverPublicKey,
                localSecretKey
              );
              if (box === null) {
                commit("incrementLoginFails");
                return;
              }
              let decryptedToken: string = base64.encode(box);
              axios
                .post("/_/auth", {
                  decryptedToken: decryptedToken,
                  username: username
                })
                .then(res => {
                  let loginValid: boolean = res.data;
                  if (loginValid) {
                    commit("login", {
                      username: username,
                      secretKey: localSecretKey
                    });
                  } else {
                    commit("incrementLoginFails");
                    return;
                  }
                })
                .catch(() => {
                  commit("incrementLoginFails");
                  return;
                });
            })
            .catch(() => {
              commit("incrementLoginFails");
              return;
            });
        })
        .catch(() => {
          commit("incrementLoginFails");
          return;
        });
    }
  }
});
