<template>
  <v-toolbar app color="primary" dark>
    <router-link to="/" active-class="" exact-active-class="" class="hidelink">
      <v-toolbar-title>
        <v-icon v-if="isLoggedIn">lock</v-icon>
        <v-icon v-else>lock_open</v-icon>
        SocLocker
      </v-toolbar-title>
    </router-link>
    <v-spacer></v-spacer>
    <v-toolbar-items>
      <v-btn to="/" exact flat>Public Feed</v-btn>
      <template v-if="isLoggedIn">
        <v-btn to="/feed" exact flat>My Feed</v-btn>
        <v-btn @click="$emit('logout')" flat>Logout</v-btn>
      </template>
      <template v-else>
        <v-btn @click="$emit('login')" flat>Login</v-btn>
        <v-btn @click="$emit('register')" flat>Register</v-btn>
      </template>
    </v-toolbar-items>
  </v-toolbar>
</template>

<script lang="ts">
import Vue from "vue";
import { mapState } from "vuex";

/**
 * Creates the Top Bar of the application. Used for navigation and actions such
 * as logging in and out.
 */
export default Vue.extend({
  name: "top-bar",
  computed: {
    ...mapState(["isLoggedIn"])
  }
});
</script>

<style lang="stylus" scoped>
.hidelink
  color inherit
  text-decoration inherit
  user-select none
</style>
