import settingsRoutes from "./panels/settings/routes";
import protocolRoutes from "./panels/protocols/routes";
import connectionsRoutes from "./panels/connection/routes";
import networkRoutes from "./panels/network/routes";
import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/",
    component: () => import("./panels/Home.vue"),
  },
  {
    path: "/overview",
    component: () => import("./panels/Overview.vue"),
  },
  ...protocolRoutes,
  connectionsRoutes,
  settingsRoutes,
  networkRoutes,
  // Always leave this as last one,
  // but you can also remove it
  // {
  //   path: "/:catchAll(.*)*",
  //   component: () => import("pages/ErrorNotFound.vue"),
  // },
];

export default createRouter({
  scrollBehavior: () => ({ left: 0, top: 0 }),
  routes,
  history: createWebHashHistory(),
});
