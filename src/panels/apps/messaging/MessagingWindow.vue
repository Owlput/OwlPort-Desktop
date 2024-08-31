<script setup lang="ts">
import { ref, Ref, nextTick, onActivated, onDeactivated } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import MessagingHistory from "./MessagingHistory.vue";
import MessagingTextbox from "./MessagingTextbox.vue";
import { useRoute } from "vue-router";
import { isBodylessHandler } from "../../../utils";

const remote = useRoute().params.id;
const message_history: Ref<Array<any>> = ref([]);
const local_peer = localStorage.getItem("local_peer_id");
const listen_handle: Ref<UnlistenFn | null> = ref(null);
function push_history(message: any) {
  message_history.value.push(message);
  nextTick(() => {
    let element = document.getElementById(`chat-history`);
    if (!element) {
      console.error("Internal state error: no 'chat-history' to select");
      return;
    }
    if (element.scrollHeight - element.scrollTop - 300 < 200) {
      document.querySelector("#chat-history > li:last-child")?.scrollIntoView();
    }
  });
}
function send(target: String, msg: any) {
  invoke("plugin:owlnest-messaging|send_msg", { target, msg }).catch(
    isBodylessHandler
  );
}
onActivated(() => {
  invoke("plugin:owlnest-messaging|get_chat_history", {
    peerId: remote,
  })
    .then((history: any) => {
      message_history.value = history;
    })
    .catch(isBodylessHandler);
  listen("owlnest-messaging-emit", (ev: any) => {
    if (ev.payload.IncomingMessage.from == remote) {
      push_history(ev.payload.IncomingMessage.msg);
    }
  })
    .then((handle) => (listen_handle.value = handle))
    .catch(isBodylessHandler);
});
onDeactivated(() => {
  if (listen_handle.value) listen_handle.value();
});
</script>

<template>
  <div id="chat-container" class="h-[100vh]">
    <section style="min-height: calc(100vh - 10rem); max-height: 60%">
      <KeepAlive>
        <MessagingHistory
          v-model:history="message_history"
          :remote="$route.params.id as string"
          :local="local_peer!"
        />
      </KeepAlive>
    </section>
    <section class="h-[10rem]">
      <Suspense>
        <MessagingTextbox
          :send_message="send"
          :push_history="push_history"
          :clear_history="
            () => {
              invoke('plugin:owlnest-messaging|clear_chat_history', {
                peerId: $route.params.id,
              });
              message_history = [];
            }
          "
          :remote="$route.params.id as string"
        />
      </Suspense>
    </section>
  </div>
</template>
