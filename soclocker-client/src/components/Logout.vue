<template>
  <v-snackbar
    v-model="logoutInfoNotice"
    :top="true"
    :timeout="3000"
    color="info"
  >
    Logged Out
  </v-snackbar>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapActions } from "vuex";

export default Vue.extend({
  name: 'logout',
  data (): {
    logoutInfoNotice: boolean
  } {
    return {
      logoutInfoNotice: false
    }
  },
  methods: {
    ...mapActions(["handleLogout"]),
    logout(): void {
      this.logoutInfoNotice = true;
      (<any>this).handleLogout()
    }
  }
})
</script>

