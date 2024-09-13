<script setup lang="ts">
import { ref, onActivated, onDeactivated, computed } from "vue";
import { Message } from "./types";
const props = defineProps(
  {
    history: Array<Message>,
    remote: { type: String, required: true },
    local: { type: String, required: true }
  }
);
let show_scroll_bottom = ref(false);
const emit = defineEmits(["update:history"]);

const history = computed({
  get() {
    return props.history;
  },
  set(value) {
    emit("update:history", value);
  },
});

onDeactivated(() => {
  removeEventListener("wheel", show_scroll_to_bottom);
});
onActivated(() => {
  addEventListener("wheel", show_scroll_to_bottom);
  scroll_to_bottom();
});
function show_scroll_to_bottom(_ev: any) {
  let element = document.getElementById(`chat-history`);
  if (!element) {
    console.error("Internal state error: no 'chat-history' to select");
    return;
  }
  if (element.scrollHeight - element.scrollTop - 350 < 200) {
    show_scroll_bottom.value = false;
  } else {
    show_scroll_bottom.value = true;
  }
}
function scroll_to_bottom() {
  let latest = document.querySelector("#chat-history > li:last-child");
  if (latest) {
    latest.scrollIntoView();
  } else {
    let wrapper = document.getElementById(`chat-history`);
    if (!wrapper) {
      console.error("Internal state error: no 'chat-history' to select");
      return;
    }
    wrapper.scroll(0, wrapper.scrollHeight);
  }
  show_scroll_bottom.value = false;
}
</script>

<template>
  <section class="w-full h-full relative">
    <ul class="flex flex-col h-full px-4 py-2 overflow-auto gutter" id="chat-history">
      <template v-for="message in history">
        <li v-if="message.from === props.remote" class="message-box bg-gray-300 self-start whitespace-pre-wrap">
          {{ message.msg }}
        </li>
        <li v-else class="message-box bg-green-300 self-end whitespace-pre-wrap">
          {{ message.msg }}
        </li>
      </template>
    </ul>
    <button v-if="show_scroll_bottom" class="absolute bottom-0 right-4 bg-transparent shadow-none border-none"
      @click="scroll_to_bottom">
      <span class="material-icons"> arrow_downward </span>
    </button>
  </section>
</template>
<style>
.message-box {
  padding: 0.25rem;
  border-radius: 0.25rem;
  border: 1px solid rgb(175, 175, 175);
  width: max-content;
  max-width: 65%;
  min-height: 2rem;
  margin: 0.25rem 0px;
}
</style>
