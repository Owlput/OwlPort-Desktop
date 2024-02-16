import kadRoutes from "./kad/routes";
export default {
  path: "network",
  component: () => import("./Layout.vue"),
  children: [
    { path: "mdns", component: () => import("./Mdns.vue") },
    kadRoutes,
    { path: "autonat", component: () => import("./Autonat.vue") },
    { path: "upnp", component: () => import("./Upnp.vue") },
    { path: "", component: () => import("./Overview.vue") },
  ],
};
