import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";
import nacl from "tweetnacl";
import * as base64 from "@stablelib/base64";
import { AuthResponse, UserResponse } from "@/model.ts";
import { getProof } from "@/auther.ts";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    username: new String(),
    publicKey: new Uint8Array(),
    secretKey: new Uint8Array(),
    userId: 0,
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
      state.userId = 0;
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
        userId
      }: {
        username: string;
        publicKey: Uint8Array;
        secretKey: Uint8Array;
        userId: number;
      }
    ) {
      state.username = username;
      state.secretKey = secretKey;
      state.publicKey = publicKey;
      state.userId = userId;
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
      return new Promise((resolve, reject) => {
        commit("logout");
        resolve();
      });
    },
    /**
     * Logs the user in based on their username and secret key.
     * @param actionContext
     * @param credentials Object containing the users username and their secret
     *   key as a base64 encoded string.
     */
    async handleLogin(
      { commit },
      {
        username,
        secretKeyB64
      }: {
        username: string;
        secretKeyB64: string;
      }
    ) {
      try {
        let localSecretKey = base64.decode(secretKeyB64)
        let box = await getProof(username, localSecretKey)

        // Encode the decrypted token
        let decryptedToken: string = base64.encode(box)

        // Ask the server to validate the token
        let loginValid: boolean = (
          await axios.post(
            "/_/auth", 
            {
              decryptedToken: decryptedToken,
              username: username
            }
          )
        ).data;

        if (loginValid) {
          // If the validation succeeds, set the login information
          let user: UserResponse = (
            await axios.get(
              "/_/user", 
              {
                params: {
                  username: username
                }
              }
            )
          ).data;
          let localPublicKey: Uint8Array = base64.decode(user.publicKey);
          commit("login", {
            username: username,
            publicKey: localPublicKey,
            secretKey: localSecretKey,
            id: user.id
          });
        } else {
          throw new Error('Login authentication was invalid')
        }
      } catch { 
        commit("incrementLoginFails");   
      }
    }
  }
});