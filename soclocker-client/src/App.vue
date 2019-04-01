<template>
  <v-app id="app">
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

export default Vue.extend({
  name: 'app',
  components: {
    Login,
    Register,
    TopBar,
    Logout
  },
  methods: {
    logout (): void {
      (<any>this.$refs.logout).logout()
    },
    login (): void {
      (<any>this.$refs.login).show()
    },
    register (): void {
      (<any>this.$refs.register).show()
    }
  }
})
</script>

<style lang="stylus">
@import '~vuetify/src/stylus/main' 
</style>
