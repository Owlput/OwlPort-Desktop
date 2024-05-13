export default {
  path: "kad",
  component: () => import("../Kad.vue"),
  children: [
    {
      path: "peers",
      component: () => import("./Peers.vue"),
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
      redirect: "/main/network/kad/peers",
    },
  ],
};
