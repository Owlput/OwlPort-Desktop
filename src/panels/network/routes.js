export default {
  path: "/network",
  component: () => import("./Layout.vue"),
  children: [
    { path: "mdns", component: () => import("./Mdns.vue") },
    { path: "", component: () => import("./Overview.vue") },
  ],
};
