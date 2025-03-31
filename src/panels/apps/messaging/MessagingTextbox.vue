<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  send_message: Function;
  push_history: Function;
  clear_history: Function;
  remote: String;
}>();
const message_text = ref("");
const send_on_enter = ref(true);
function send() {
  if (!message_text.value) {
    return;
  }
  props.push_history({ msg: message_text.value });
  props.send_message(props.remote, message_text.value);
  message_text.value = "";
}
</script>
<template>
  <section class="h-full w-full border-t">
    <textarea v-model="message_text" class="resize-none border w-full p-4" style="height: calc(100% - 3.5rem)"
      @keydown.enter.exact.prevent="() => {
        if (!send_on_enter) message_text = message_text + `\n`;
      }
        " @keyup.enter.exact.prevent="() => {
          if (send_on_enter) send();
        }
          "></textarea>
    <section class="flex justify-between items-center px-8 select-none h-[3rem]">
      <ul class="flex flex-row">
        <li class="hover:bg-slate-100 active:bg-slate-300 text" @click="() => props.clear_history()">
          <span class="material-icons m-auto">delete_forever</span>
        </li>
        <li>
          <input type="checkbox" v-model="send_on_enter" />
          <p class="float-right">Send on Enter</p>
        </li>
      </ul>
      <button class="w-[4rem] h-10" @click="send">Send</button>
    </section>
  </section>
</template>
<style scoped></style>
