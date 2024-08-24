<script setup>
import { ref, nextTick, onActivated, onDeactivated } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import MessagingHistory from "./MessagingHistory.vue";
import MessagingTextbox from "./MessagingTextbox.vue";
import { useRoute } from "vue-router";
const remote = useRoute().params.id;
const message_history = ref([]);
const local_peer = localStorage.getItem("local_peer_id");
const listen_handle = ref(null);
function push_history(message) {
  message_history.value.push(message);
  nextTick(() => {
    let element = document.getElementById(`chat-history`);
    if (element.scrollHeight - element.scrollTop - 300 < 200) {
      document.querySelector("#chat-history > li:last-child")?.scrollIntoView();
    }
  });
}
function send(target, msg) {
  invoke("plugin:owlnest-messaging|send_msg", { target, msg });
}
onActivated(() => {
  invoke("plugin:owlnest-messaging|get_chat_history", {
    peerId: remote,
  }).then((history) => {
    message_history.value = history;
  });
  listen("owlnest-messaging-emit", (ev) => {
    if (ev.payload.IncomingMessage.from == remote) {
      push_history(ev.payload.IncomingMessage.msg);
    }
  }).then((handle) => (listen_handle.value = handle));
});
onDeactivated(() => {
  listen_handle.value();
});
</script>

<template>
  <div id="chat-container" class="h-[100vh]">
    <section style="min-height: calc(100vh - 10rem); max-height: 60%;">
      <KeepAlive>
        <MessagingHistory
          v-model:history="message_history"
          :remote="$route.params.id"
          :local="local_peer"
        />
      </KeepAlive>
    </section>
    <section class="h-[10rem]">
      <Suspense>
        <MessagingTextbox
          :send_message="send"
          :push_history="push_history"
          :clear_history="() => {invoke('plugin:owlnest-messaging|clear_chat_history',{peerId: $route.params.id});message_history = []}"
          :remote="$route.params.id"
        />
      </Suspense>
    </section>
  </div>
</template>
