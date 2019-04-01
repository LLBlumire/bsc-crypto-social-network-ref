<template>
  <div class="register">

    <v-alert
      app
      type="warning"
      v-model="showRegisteredSecretKey"
      :dismissible="true"
    >
      You have registered an account with the Secret Key<br/>
      <b class="monospace">{{ registeredSecretKey }}</b><br/>
      Ensure that you store this key securely as you cannot recover or change
      it.
    </v-alert>

    <v-snackbar
      v-model="registerSuccessNotice"
      :top="true"
      :timeout="3000"
      color="success"
    >
      Registered as {{ username }}!
    </v-snackbar>

    <v-snackbar
      v-model="registerErrorNotice"
      :top="true"
      :timeout="3000"
      color="error"
    >
      Something went wrong with registration, please try again later.
    </v-snackbar>

    <v-dialog v-model="showRegister" content-class="REGISTER-INSTANCE-DIALOG">
      <v-card>
        <v-card-title class="headline" primary-title>
          Register
        </v-card-title>

        <v-form ref="form" v-model="valid" @submit.prevent="register">

          <v-card-text>

            <v-container grid-list-md>
              <v-flex xs12>
                <v-text-field 
                  label="Username"
                  name="username"
                  v-model="username"
                  :rules="usernameRules" 
                ></v-text-field>
              </v-flex>
              <v-flex xs12>
                <v-btn
                  block
                  color="secondary"
                  flat
                  @click="genKey"
                >
                  Generate Keys
                </v-btn>
              </v-flex>
              <v-flex xs12>
                <v-text-field
                  class="monospace"
                  label="Public Key"
                  readonly
                  v-model="publicKey"
                ></v-text-field>
              </v-flex>
              <v-flex xs12>
                <v-text-field
                  class="monospace"
                  name="password"
                  label="Secret Key"
                  readonly
                  v-model="secretKey"
                ></v-text-field>
              </v-flex>
              
            </v-container>
            <v-alert type="warning" :value="true">
              Above you have created a Secret Key. This acts as your password 
              for SocLocker and <b>MUST</b> be kept stored securely. If your key
              is lost there is <b>NO WAY TO RECOVER IT</b>. As such, it is
              recommended to use SocLocker in combination with a password
              manager, such as
              <a href="https://www.dashlane.com/">Dashlane</a> or
              <a href="https://www.lastpass.com/">LastPass</a>.
            </v-alert>

          </v-card-text>

          <v-card-actions>
            <v-spacer />
            <v-btn 
              type="submit"
              color="primary"
              :disabled="!valid"
            >
              Register
            </v-btn>
          </v-card-actions>
        </v-form>
      </v-card>
    </v-dialog>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import axios from 'axios'
import nacl from 'tweetnacl'
import * as base64 from "@stablelib/base64";
import { mapState, mapActions } from 'vuex';

export default Vue.extend({
  name: 'register',
  data (): {
    username: string,
    publicKey: string,
    secretKey: string,
    usernameExists: boolean,
    showRegister: boolean,
    valid: boolean,
    registerSuccessNotice: boolean,
    registerErrorNotice: boolean,
    registeredSecretKey: string,
  } {
    return {
      username: "",
      publicKey: "",
      secretKey: "",
      usernameExists: false,
      showRegister: false,
      valid: false,
      registerSuccessNotice: false,
      registerErrorNotice: false,
      registeredSecretKey: ""
    }
  },
  computed: {
    ...mapState([
      'isLoggedIn',
      'loginAttemptFails'
    ]),
    /**
     * Determines the rules for username validation.
     * 1. A username must not be empty
     * 2. A username must not have exactly 44 characters
     * 3. A username must not be a duplicate of one already registered
     */
    usernameRules (): ((v: string) => boolean | string)[] {
      return [
        (v: string) => !!v || 'Username is Required',
        (v: string) => v.length !== 44 || 'Username resembles Secret Key',
        (v: string) => !this.usernameExists || 'Username already exists'
      ]
    },

    /**
     * Determines if the alert notifying of a secret key registration should
     * be displayed. That is to say to display the secret key one final time
     * if the user has registered in a given session.
     */
    showRegisteredSecretKey: {
      get(): boolean {
        return this.registeredSecretKey !== ""
      },
      set(show: boolean): void {
        if (!show) {
          this.registeredSecretKey = ""
        }
      }
    }
  },
  watch: {
    /**
     * On changes to the username, attempt to validate the username and verify
     * that it does not already exist.
     */
    username (username) {
      axios.get('/_/user', { params: { username: username } }).then(() => {
        this.usernameExists = true;
        (<any>this.$refs.form).validate()
      }).catch(() => {
        this.usernameExists = false;
        (<any>this.$refs.form).validate()
      })
    }
  },
  methods: {
    ...mapActions([
      'handleLogin'
    ]),

    /**
     * Displays the registration dialog.
     */
    show() {
      this.showRegister = true;
      (<any>this.$refs.form).validate()
      this.genKey();
    },

    /**
     * Registers the account
     */
    register() {
      axios.post('/_/user', {
        username: this.username,
        publicKey: this.publicKey
      }).then(() => {
        this.showRegister = false
        this.registerSuccessNotice = true
        this.registeredSecretKey = this.secretKey
      }).catch(() => {
        this.registerErrorNotice = true
      })
    },

    /**
     * Generates a new keypair for registration.
     */
    genKey() {
      let { publicKey, secretKey } = nacl.box.keyPair()
      this.publicKey = base64.encode(publicKey)
      this.secretKey = base64.encode(secretKey)
    }
  }
})
</script>

<style lang="stylus">
.REGISTER-INSTANCE-DIALOG
  max-width 800px
</style>

<style lang="stylus" scoped>
.monospace
  font-family monospace
</style>

