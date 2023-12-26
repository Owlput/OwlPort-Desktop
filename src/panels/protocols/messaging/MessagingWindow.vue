<script setup>
import { ref, onUnmounted } from "vue";
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
onUnmounted(()=>{console.log("messaging window destroyed")})
</script>

<template>
  <div class="grid grid-cols-1 grid-rows-2 h-full" id="chat-container">
    <section class="h-full overflow-auto">
      <KeepAlive>
        <MessagingHistory v-model:history="message_history" :remote="$route.params.id"/>
      </KeepAlive>
    </section>
    <Suspense>
      <MessagingTextbox
        :send_message="send"
        :push_history="push_history"
        :clear_history="()=>message_history = []"
        :remote="$route.params.id"
      />
    </Suspense>
  </div>
</template>
<style>
#chat-container {
  height: calc(100vh - 3.5rem);
  grid-template-rows: auto 12rem;
}
</style>
