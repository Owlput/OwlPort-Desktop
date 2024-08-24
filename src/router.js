import settingsRoutes from "./panels/settings/routes";
import messagingApp from "./panels/protocols/messaging/routes";
import connectionsRoutes from "./panels/connection/routes";
import networkRoutes from "./panels/network/routes";
import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/main",
    component: () => import("./MainWindow.vue"),
    children: [
      connectionsRoutes,
      settingsRoutes,
      networkRoutes,
      {
        path: "apps",
        component: () => import("./panels/protocols/ProtocolOverview.vue"),
      },
      {
        path: "overview",
        component: () => import("./panels/Overview.vue"),
      },
      {
        path: "home",
        component: () => import("./panels/Home.vue"),
      },
    ],
  },
  {
    path: "/app",
    component: () => import("./AppWrapper.vue"),
    children: [
      messagingApp,
      {
        path: "blob-transfer",
        component: () =>
          import("./panels/protocols/blob_transfer/BlobTransfer.vue"),
      },
    ],
  },
  {
    path: "/",
    redirect: "/main/home",
  },
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
