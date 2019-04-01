import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";
import nacl from "tweetnacl";
import * as base64 from "@stablelib/base64";
import { AuthResponse, UserResponse } from "@/model.ts";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    username: new String(),
    publicKey: new Uint8Array(),
    secretKey: new Uint8Array(),
    loginAttemptFails: 0,
    isLoggedIn: false
  },
  mutations: {
    /**
     * Log the user out of the website.
     * @param state Internal vuex state.
     */
    logout(state) {
      state.username = new String();
      state.secretKey = new Uint8Array();
      state.publicKey = new Uint8Array();
      state.loginAttemptFails = 0;
      state.isLoggedIn = false;
    },

    /**
     * Logs the user into the website
     * @param state Internal vuex state.
     * @param credentials Object containing the users username and their secret 
     *   key as a byte array. 
     */
    login(
      state,
      {
        username,
        publicKey,
        secretKey,
      }: {
        username: string;
        publicKey: Uint8Array;
        secretKey: Uint8Array;
      }
    ) {
      state.username = username;
      state.secretKey = secretKey;
      state.publicKey = publicKey;
      state.loginAttemptFails = 0;
      state.isLoggedIn = true;
    },

    /**
     * Increments the number of times a login has failed.
     * @param state Internal vuex state.
     */
    incrementLoginFails(state) {
      state.loginAttemptFails += 1;
    }
  },
  actions: {
    /**
     * Logs the user out.
     * @param actionContext Internal vuex context.
     */
    handleLogout({ commit }) {
      commit("logout");
    },
    /**
     * Logs the user in based on their username and secret key.
     * @param actionContext
     * @param credentials Object containing the users username and their secret
     *   key as a base64 encoded string.
     */
    handleLogin(
      { commit },
      {
        username,
        secretKeyB64,
      }: {
        username: string;
        secretKeyB64: string;
      }
    ) {
      // Acquire the servers public key
      axios
        .get("/_/server_public_key")
        .then(spk => {
          let serverPublicKeyB64: string = spk.data;
          // Have the server generate a challenge proof
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
              // Send the server the completed challenge as proof of identity.
              axios
                .post("/_/auth", {
                  decryptedToken: decryptedToken,
                  username: username
                })
                .then(res => {
                  let loginValid: boolean = res.data;
                  if (loginValid) {
                    // Get the users public key
                    axios.get("/_/user", {
                      params: {
                        username: username
                      }
                    })
                    .then((res) => {
                      let user = <UserResponse>res.data;
                      let localPublicKey = base64.decode(user.publicKey);
                      commit("login", {
                        username: username,
                        publicKey: localPublicKey,
                        secretKey: localSecretKey,
                      });  
                    })
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
