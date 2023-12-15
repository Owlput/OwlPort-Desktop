import messaging_routes from './messaging/routes'
export default {
    path: "/protocols",
    component: () => import("./ProtocolOverview.vue"),
    children: [
      messaging_routes,
    ],
  };
  