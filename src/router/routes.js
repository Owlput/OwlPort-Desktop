import settingsPage from "./settingsPage";

const routes = [
  {
    path: "/",
    component: () => import("../pages/Home.vue"),
  },
  {
    path: "/overview",
    component: () => import("../pages/overview/Overview.vue"),
  },
  {
    path: "/apps",
    component: () => import("../layouts/Applications.vue"),
  },
  settingsPage,

  // Always leave this as last one,
  // but you can also remove it
  // {
  //   path: "/:catchAll(.*)*",
  //   component: () => import("pages/ErrorNotFound.vue"),
  // },
];

export default routes;
