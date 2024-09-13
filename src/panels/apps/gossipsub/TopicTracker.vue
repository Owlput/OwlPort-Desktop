<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api";
import SearchBar from "../../../components/SearchBar.vue";
import { Message, ReadableTopic } from "./types";
import { isBodylessHandler } from "../../../utils";

const topic_to_track = ref("")
const send_on_enter = ref(false)
const message_history: Ref<Array<Message>> = ref([]);
const message = ref("")
const utf8Decoder = new TextDecoder();

function update_topic_history() {
  if (topic_to_track.value.length === 0) return;
  if (topic_to_track.value[0] === '#') {
    invoke<Array<any>>("plugin:libp2p-gossipsub|get_message_history", { topic: { HashOnly: topic_to_track.value } }).then(
      (v) => message_history.value = v.map((v) => new Message(v.data, v.topic, v.sequence_number, v.source))
    );
  } else {
    invoke<Array<any>>("plugin:libp2p-gossipsub|get_message_history", { topic: { StringOnly: topic_to_track.value } }).then(
      (v) => message_history.value = v.map((v) => new Message(v.data, v.topic, v.sequence_number, v.source))
    );
  }
}
function send_message() {
  if (topic_to_track.value.length === 0 || message.value.length === 0) return;
  let topic = topic_to_track.value.startsWith('#') ? new ReadableTopic({ HashOnly: { hash: topic_to_track.value } }) : new ReadableTopic({ StringOnly: topic_to_track.value });
  invoke("plugin:libp2p-gossipsub|publish_message", { topic, message: message.value }).then((v) => { console.log(v) }).catch(isBodylessHandler);
}


</script>
<template>
  <div class="w-[100vw]">
    <SearchBar v-model="topic_to_track" place-holder="Type the topic or hash(start with #) to track here"
      :refresh="update_topic_history" @search-submit="update_topic_history"></SearchBar>
    <ul style="height: calc(100vh - 15.5rem);" class="message-list">
      <li v-for="msg in message_history" class="rounded-md">
        <p>From: {{ msg.source }}</p>
        <p>Message: {{ utf8Decoder.decode(msg.data) }}</p>
      </li>
    </ul>

    <section class="h-full w-full border-t">
      <textarea v-model="message" class="resize-none border w-full p-4" style="height: calc(100% - 3.5rem)"
        @keydown.enter.exact.prevent="() => {
          if (!send_on_enter) message += `\n`;
        }
          " @keyup.enter.exact.prevent="() => {
            if (send_on_enter) send_message();
          }
            "></textarea>
      <section class="flex justify-between items-center px-8 select-none h-[3rem]">
        <ul class="flex flex-row">
          <li class="hover:bg-slate-100 active:bg-slate-300 text" @click="() => { }">
            <span class="material-icons m-auto">delete_forever</span>
          </li>
          <li>
            <input type="checkbox" v-model="send_on_enter" />
            <p class="float-right">Send on Enter</p>
          </li>
        </ul>
        <button class="w-[4rem] h-10" @click="send_message">Send</button>
      </section>
    </section>
  </div>
</template>

<style>
.message-list{
  display: flex;
  padding: 1rem;
}
.message-list > li{
  background-color: rgb(155, 245, 155);
  width: 100%;
  height: fit-content;
  padding: 0.5rem 1rem;
}
</style>