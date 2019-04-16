<template>
  <v-app id="app">
    <!-- TopBar emits events for each UI button it exposes. -->
    <top-bar @login="login" @register="register" @logout="logout"/>
    <v-content>
      <login ref="login" />
      <register ref="register" />
      <logout ref="logout" />
      <v-container>
        <v-layout>
          <v-flex 
            xs12 
            lg10 offset-lg1
            xl8 offset-xl2
          >
            <router-view @register="register" />
          </v-flex>
        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import Login from '@/components/Login.vue'
import Register from '@/components/Register.vue'
import Logout from '@/components/Logout.vue'
import TopBar from '@/components/TopBar.vue'
import { mapActions } from 'vuex'

/**
 * Represents the application as a whole, composing each element loaded within
 * it.
 */
export default Vue.extend({
  name: 'app',
  components: {
    Login,
    Register,
    TopBar,
    Logout
  },
  computed: {
    ...mapState(["isLoggedIn"])
  },
  methods: {
    /**
     * Active the logout method of the Logout component.
     * This logs the user out.
     */
    logout (): void {
      (<any>this.$refs.logout).logout()
    },

    /**
     * Show the login dialog.
     */
    login (): void {
      (<any>this.$refs.login).show()
    },

    /**
     * Show the register dialog.
     */
    register (): void {
      (<any>this.$refs.register).show()
    }
  },
  watch: {
    /**
     * Handle changing the page when the user logs in and out.
     */
    isLoggedIn(isLoggedIn) {
      if (isLoggedIn) {
        this.$router.push({ name: 'feed' })
      } else {
        this.$router.push({ name: 'public' })
      }
    }
  }
})
</script>

<style lang="stylus">
@import '~vuetify/src/stylus/main' 
</style>
