<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import MessagingHistory from "./MessagingHistory.vue";
import MessagingTextbox from "./MessagingTextbox.vue";

let message_history = ref([]);
function push_history(message) {
  message_history.value.push(message);
}
function send(target, msg) {
  invoke("plugin:owlnest-messaging|send_msg", { target, msg });
}
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-2 h-auto m-0" id="chat-container">
    <section>
      <KeepAlive>
        <MessagingHistory :message_history="message_history" />
      </KeepAlive>
    </section>
    <Suspense>
      <MessagingTextbox
        :send_message="send"
        :push_history="push_history"
        :remote="$route.params.id"
      />
    </Suspense>
  </div>
</template>
<style>
#chat-container {
  grid-template-rows: 70% 30%;
}
</style>
