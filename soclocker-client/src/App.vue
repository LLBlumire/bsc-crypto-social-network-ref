<template>
  <v-app id="app">
    <!-- TopBar emits events for each UI button it exposes. -->
    <top-bar @login="login" @register="register" @logout="logout"/>
    <v-content>
      <login ref="login" />
      <register ref="register" />
      <logout ref="logout" />
      <router-view />
    </v-content>
  </v-app>
</template>

<script lang="ts">
import Vue from 'vue'
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
  }
})
</script>

<style lang="stylus">
@import '~vuetify/src/stylus/main' 
</style>
