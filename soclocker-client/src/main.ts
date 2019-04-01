/**
 * This program specifies a reference implementation of the front end of a 
 * social media service that has no server-side knowledge of its users 
 * authentication credentials.
 */

import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import Vuetify from "vuetify";

Vue.use(Vuetify);
Vue.config.productionTip = false;

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");
