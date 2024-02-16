<script setup>
import { ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import MessagingHistory from "./MessagingHistory.vue";
import MessagingTextbox from "./MessagingTextbox.vue";
import { useRoute } from "vue-router";
const route = useRoute();
let message_history = ref([]);
function push_history(message) {
  message_history.value.push(message);
  nextTick(() => {
    let element = document.getElementById(`chat${route.params.id}`);
    if (element.scrollHeight - element.scrollTop - 300 < 200) {
      document.getElementById("last-message")?.scrollIntoView();
    }
  });
}
function send(target, msg) {
  invoke("plugin:owlnest-messaging|send_msg", { target, msg });
}
</script>

<template>
  <div class="h-full" id="chat-container">
    <section class="h-[60%] overflow-auto">
      <KeepAlive>
        <MessagingHistory
          v-model:history="message_history"
          :remote="$route.params.id"
        />
      </KeepAlive>
    </section>
    <section class="h-[40%]">
      <Suspense>
        <MessagingTextbox
          :send_message="send"
          :push_history="push_history"
          :clear_history="() => (message_history = [])"
          :remote="$route.params.id"
        />
      </Suspense>
    </section>
  </div>
</template>
