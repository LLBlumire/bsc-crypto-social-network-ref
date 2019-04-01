import Vue from "vue";
import Router from "vue-router";
import PublicFeed from "./views/PublicFeed.vue";

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: "/",
      name: "public-feed",
      component: PublicFeed
    }
  ]
});
