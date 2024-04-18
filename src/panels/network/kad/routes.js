export default {
  path: "kad",
  component: () => import("../Kad.vue"),
  children: [
    {
      path: "activity",
      component: () => import("./Activity.vue"),
    },
    {
      path: "query",
      component: () => import("./Query.vue"),
    },
    {
      path: "bootstrap",
      component: () => import("./Bootstrap.vue"),
    },
    {
      path: "",
      redirect: "/main/network/kad/activity",
    },
  ],
};
