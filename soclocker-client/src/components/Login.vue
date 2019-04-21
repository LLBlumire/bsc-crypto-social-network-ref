<template>
  <div class="login">
    <v-snackbar
      v-model="loginFailNotice"
      :top="true"
      :timeout="3000"
      color="error"
    >
      Login Failed!
    </v-snackbar>

    <v-snackbar
      v-model="loginWarnNotice"
      :top="true"
      :timeout="3000"
      color="warning"
    >
      Ensure Login Form is Valid
    </v-snackbar>

    <v-snackbar
      v-model="loginSuccessNotice"
      :top="true"
      :timeout="3000"
      color="success"
    >
      Logged in as {{ username }}!
    </v-snackbar>

    <v-dialog v-model="showLogin" content-class="LOGIN-INSTANCE-DIALOG">
      <v-card>
        <v-card-title class="headline" primary-title>
          Login
        </v-card-title>
        <v-form @submit.prevent="login" v-model="validLoginForm">
          <v-card-text>
            <v-container grid-list-md>
              <v-flex xs12>
                <v-text-field
                  name="username"
                  label="Username"
                  v-model="username"
                  :rules="usernameRules"
                ></v-text-field>
              </v-flex>
              <v-flex xs12>
                <v-text-field
                  name="password"
                  label="Secret Key"
                  v-model="secretKey"
                  :type="revealSecretKey ? 'text' : 'password'"
                  :append-icon="
                    revealSecretKey ? 'visibility' : 'visibility_off'
                  "
                  @click:append="revealSecretKey = !revealSecretKey"
                  :rules="secretKeyRules"
                ></v-text-field>
              </v-flex>
            </v-container>
          </v-card-text>
          <v-card-actions>
            <v-spacer />
            <v-btn type="submit" color="primary" flat :loading="loggingIn">
              Login
            </v-btn>
          </v-card-actions>
        </v-form>
      </v-card>
    </v-dialog>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { mapState, mapActions } from "vuex";

/**
 * Controls the login dialogue and performs the login operations. Most procedure
 * of the login operation is offloaded to the vuex store.
 */
export default Vue.extend({
  name: "login",
  data(): {
    username: string;
    secretKey: string;
    loginFailNotice: boolean;
    loginWarnNotice: boolean;
    loginSuccessNotice: boolean;
    revealSecretKey: boolean;
    usernameRules: ((v: string) => boolean | string)[];
    secretKeyRules: ((v: string) => boolean | string)[];
    validLoginForm: boolean;
    showLogin: boolean;
    loggingIn: boolean;
  } {
    return {
      username: "",
      secretKey: "",
      loginFailNotice: false,
      loginWarnNotice: false,
      loginSuccessNotice: false,
      revealSecretKey: false,
      usernameRules: [
        (v: string) => !!v || "Username is Required",
        (v: string) => v.length !== 44 || "Username resembles Secret Key"
      ],
      secretKeyRules: [(v: string) => !!v || "Secret Key is required"],
      validLoginForm: true,
      showLogin: false,
      loggingIn: false
    };
  },
  computed: {
    ...mapState(["isLoggedIn", "loginAttemptFails"])
  },
  methods: {
    ...mapActions(["handleLogin"]),

    /**
     * Initiates the login process.
     */
    async login() {
      this.loggingIn = true;
      if (!this.validLoginForm) {
        this.loginWarnNotice = true;
        return;
      }
      await this.handleLogin({
        username: this.username,
        secretKeyB64: this.secretKey
      });
      this.loggingIn = false;
    },

    /**
     * Opens the login dialog.
     */
    show() {
      this.showLogin = true;
    }
  },
  watch: {
    /**
     * On an incrementing of the failed login count, display the failed login
     * notice.
     */
    loginAttemptFails(newLoginAttemptFails, oldLoginAttemptFails) {
      if (newLoginAttemptFails > oldLoginAttemptFails) {
        this.loginFailNotice = true;
      }
    },

    /**
     * Dismisses the login dialog if the user becomes logged in. As well as
     * notifying the user of their successful login.
     */
    isLoggedIn(loggedIn) {
      if (loggedIn) {
        this.loginSuccessNotice = true;
        this.showLogin = false;
      }
    }
  }
});
</script>

<style lang="stylus">
.LOGIN-INSTANCE-DIALOG
  max-width 800px
</style>
