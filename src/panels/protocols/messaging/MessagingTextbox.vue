<script setup>
import { listen } from "@tauri-apps/api/event";
import { onUnmounted, ref } from "vue";
const props = defineProps({
  send_message: Function,
  push_history: Function,
  clear_history: Function,
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
    <textarea v-model="message" class="resize-none border w-full h-[9rem]"></textarea>
    <section class="flex justify-between items-center px-8 select-none">
      <ul class="flex flex-row">
        <li class="hover:bg-slate-100 active:bg-slate-300 text" @click="props.clear_history"><span class="material-icons m-auto">delete_forever</span></li>
      </ul>
      <button class="w-[4rem] h-10" @click="send">Send</button>
    </section>
  </section>
</template>
<style scoped></style>
