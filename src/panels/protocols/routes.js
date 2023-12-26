import messaging_routes from "./messaging/routes";
export default [
  {
    path: "/protocols",
    component: () => import("./ProtocolOverview.vue"),
  },
  messaging_routes,
  {
    path: "/protocols/blob-transfer",
    component: () => import("./blob_transfer/BlobTransfer.vue"),
  },
];
