export default {
    path: "/connections",
    component: () => import("./ConnectionWrapper.vue"),
    children: [
      {
        path: "connected",
        component: () => import("./ConnectedPeers.vue"),
      },
      {
        path: "dial",
        component: () => import("./Dial.vue"),
      },
      {
        path: "",
        component: () => import("./Dial.vue"),
      },
    ],
  };
  