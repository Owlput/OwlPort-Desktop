<script setup lang="ts">
import { ref, onActivated, onDeactivated, Ref, nextTick, computed } from "vue";
import { Message, MessagingEmit } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { isBodylessHandler } from "../../../../utils";
import { listen } from "@tauri-apps/api/event";
const props = defineProps(
  {
    history: { type: Array<Message>, required: true },
    remote: { type: String, required: true },
  }
);

const emit = defineEmits(["update:history"]);
const history = computed({
  get() {
    return props.history;
  },
  set(value) {
    emit("update:history", value);
  },
});


const listen_handle: Ref<Function> = ref(() => { });
const show_scroll_bottom = ref(false);

function scroll_to_view() {
  nextTick(() => {
    let element = document.getElementById(`chat-history`);
    if (!element) {
      console.error("Internal state error: no 'chat-history' to select");
      return;
    }
    if ((element.scrollHeight - element.scrollTop - window.innerHeight - 200) < 200) {
      document.querySelector("#chat-history > li:last-of-type")?.scrollIntoView();
    }
  });
}

onDeactivated(() => {
  removeEventListener("wheel", show_scroll_to_bottom);
  if (listen_handle.value) listen_handle.value();
});
onActivated(() => {
  get_chat_history()
  listen<MessagingEmit>("owlnest-messaging-emit", (ev) => {
    if (ev.payload.IncomingMessage?.from == props.remote) {
      history.value.push(ev.payload.IncomingMessage.msg);
      scroll_to_view()
    }
  })
    .then((handle) => (listen_handle.value = handle))
    .catch(isBodylessHandler);
  addEventListener("wheel", show_scroll_to_bottom);
});

function get_chat_history() {
  invoke<Array<Message>>("plugin:owlnest-messaging|get_chat_history", {
    peerId: props.remote,
  })
    .then((v) => {
      history.value = v;
      nextTick(scroll_to_bottom)
    })
    .catch(isBodylessHandler);
}

function show_scroll_to_bottom(_ev: any) {
  let element = document.getElementById(`chat-history`);
  if (!element) {
    console.error("Internal state error: no 'chat-history' to select");
    return;
  }
  let scroll_pos = element.scrollHeight - element.scrollTop;
  if (scroll_pos - 350 > 200 && scroll_pos > element.offsetHeight) {
    show_scroll_bottom.value = true;
  } else {
    show_scroll_bottom.value = false;
  }
}
function scroll_to_bottom() {
  let latest = document.querySelector("#chat-history > li:last-of-type");
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
    <ul class="flex flex-col px-4 py-2 overflow-auto gutter" style="height: calc(100vh - 10rem);" id="chat-history">
      <template v-for="message in history">
        <li v-if="message.from === props.remote" class="message-box bg-gray-300 self-start">
          <p>{{ message.msg }}</p>
        </li>
        <li v-else class="message-box bg-green-300 self-end ">
          <p>{{ message.msg }}</p>
        </li>
      </template>
    </ul>
    <button v-if="show_scroll_bottom" class="absolute bottom-0 right-[50%] bg-transparent shadow-none border-none"
      @click="scroll_to_bottom">
      <span class="material-icons"> arrow_downward </span>
    </button>
  </section>
</template>
<style>
.message-box {
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  border: 1px solid rgb(200, 200, 200);
  width: max-content;
  max-width: 65%;
  margin: 0.25rem 0px;
}
</style>
