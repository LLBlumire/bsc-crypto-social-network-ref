import Vue from "vue";
import Router from "vue-router";
import PublicView from "./views/PublicView.vue";
import Feed from "./views/Feed.vue";

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: "/",
      name: "public",
      component: PublicView
    },
    {
      path: "/feed",
      name: "feed",
      component: Feed
    }
  ]
});
