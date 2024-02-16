export default {
  path: "settings",
  component: () => import("./SettingsOverview.vue"),
  children: [
    {
      path: "certs-and-keys",
      component: () => import("./CertsAndKeys.vue"),
    },
    {
      path: "general",
      component: () => import("./General.vue"),
    },
    {
      path: "",
      component: () => import("./General.vue"),
    },
  ],
};
