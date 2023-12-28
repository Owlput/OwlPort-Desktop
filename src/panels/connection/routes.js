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
      path: "listen",
      component: () => import("./Listen.vue"),
    },
    {
      path: "relays",
      component: () => import("./Relay.vue"),
    },
    {
      path: "",
      redirect: "/connections/dial",
    },
  ],
};
