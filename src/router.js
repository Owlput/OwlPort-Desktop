import settingsRoutes from "./panels/settings/routes";
import messagingApp from "./panels/apps/messaging/routes";
import gossipsubApp from "./panels/apps/gossipsub/routes";
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
        component: () => import("./panels/apps/ProtocolOverview.vue"),
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
    component: () => import("./panels/apps/App.vue"),
    children: [
      messagingApp,
      {
        path: "blob-transfer",
        component: () => import("./panels/apps/blob_transfer/BlobTransfer.vue"),
      },
      gossipsubApp,
    ],
  },
  {
    path: "/dev",
    component: () => import("./components/EmptyWindow.vue"),
    children: [
      {
        path: "swarm-event-listener",
        component: () => import("./panels/settings/SwarmEventListener.vue"),
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
