<script setup>
import { listen } from "@tauri-apps/api/event";
import { onUnmounted, ref } from "vue";
const props = defineProps({
  send_message: Function,
  push_history: Function,
  remote: String,
});
let message = ref("");
let incoming_handle = await listen("owlnest-messaging-emit", (ev) => {
  if (ev.payload.IncomingMessage.from == props.remote) {
    props.push_history(ev.payload.IncomingMessage.msg);
  }
});
onUnmounted(() => {
  incoming_handle();
});
function send() {
  if (!message.value){
    return;
  }
  props.push_history({ msg:message.value });
  props.send_message(props.remote, message.value);
  message.value = "";
}
</script>
<template>
  <section class="h-full w-full border-t">
    <textarea v-model="message" class="resize-none border"></textarea>
    <section class="flex justify-between px-8">
      <ul class="flex flex-row">
        <li>A</li>
        <li>B</li>
        <li>C</li>
      </ul>
      <button class="w-[5rem] h-10" @click="send">Send</button>
    </section>
  </section>
</template>
<style scoped></style>
