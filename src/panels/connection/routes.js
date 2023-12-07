export default {
    path: "/connections",
    component: () => import("./ConnectionOverview.vue"),
    children: [
      {
        path: "connected",
        component: () => import("./ConnectedPeers.vue"),
      },
      {
        path: "",
        component: () => import("./ConnectionOverview.vue"),
      },
    ],
  };
  