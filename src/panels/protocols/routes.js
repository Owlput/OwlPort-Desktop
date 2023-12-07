export default {
    path: "/protocols",
    component: () => import("./ProtocolOverview.vue"),
    children: [
      {
        path: "messaging",
        component: () => import("./Messaging.vue"),
      },
      {
        path: "",
        component: () => import("./ProtocolOverview.vue"),
      },
    ],
  };
  