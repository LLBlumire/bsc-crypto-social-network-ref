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

/**
 * Handles logout notifications and the logout process.
 */
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
    /**
     * Logs the user out.
     */
    logout(): void {
      this.logoutInfoNotice = true;
      (<any>this).handleLogout()
    }
  }
})
</script>

