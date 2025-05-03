const chat_route = {
  path: "chat",
  component: () => import("./Chat.vue"),
  children: [
    {
      path: ":peerId",
      component: () => import("./components/ChatLayout.vue"),
    },
    {
      path: "",
      component: () => import("./components/EmptyChat.vue"),
    },
  ],
};

export default {
  path: "messaging",
  component: () => import("./Messaging.vue"),
  children: [
    {
      path: "",
      component: () => import("./Inbox.vue"),
    },
    chat_route,
    {
      path: "contacts/:peerId?",
      component: () => import("./Contacts.vue"),
    },
    {
      path: "discover/:peerId?",
      component: () => import("./Discover.vue"),
    },
    {
      path: "settings",
      component: () => import("./Settings.vue"),
    },
  ],
};
