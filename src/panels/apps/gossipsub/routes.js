export default {
  path: "gossipsub",
  component: () => import("./GossipSub.vue"),
  children: [
    {
      path: "all_topics",
      component: () => import("./AllTopics.vue"),
    },
    {
      path: "subscription",
      component: () => import("./Subscription.vue"),
    },
    {
      path: "topic_tracker",
      component: () => import("./TopicTracker.vue"),
    },
    {
      path: "",
      redirect: "/app/gossipsub/all_topics",
    },
  ],
};
