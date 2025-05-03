<script setup lang="ts">
import { onMounted, onUnmounted, ref, Ref, ShallowRef, shallowRef } from "vue";
import { invoke } from "@tauri-apps/api/core";
import * as types from "./types";
import { isBodylessHandler } from "../../../utils";
import { TopicHash } from "./types";

const topic_to_track: Ref<string> = ref("")
const hash_type: Ref<string> = ref("Identity");
const hash_type_items = [{ name: 'Identity', value: "identity" }, { name: 'SHA256', value: "sha256" }];

const message_history: ShallowRef<Array<types.MessageRecord>> = shallowRef([]);
const utf8Decoder = new TextDecoder();
const refresh_interval_handle: Ref<number | null> = ref(null);

function update_topic_history() {
  if (topic_to_track.value.length === 0) return;
  let param;
  if (topic_to_track.value[0] === '#') {
    param = { topic: types.Topic.hash_only(new TopicHash(topic_to_track.value.slice(1))) }
  } else {
    param = { topic: types.Topic.sha256_string(topic_to_track.value) }
  }
  invoke<Array<any>>("plugin:owlnest-gossipsub|get_message_history", param).then(
    (v) => message_history.value = v ? v.map((v) => types.MessageRecord.deserialize(v)) : []
  );
}

const send_on_enter = ref(false)
const message = ref("")
function send_message() {
  if (topic_to_track.value.length === 0 || message.value.length === 0) return;
  let topic = topic_to_track.value.startsWith('#') ? types.Topic.hash_only(new TopicHash(topic_to_track.value.slice(1))) : types.Topic.sha256_string(topic_to_track.value);
  invoke("plugin:owlnest-gossipsub|publish_message", { topic, message: message.value }).then((v) => { console.log(v) }).catch(isBodylessHandler);
}

onMounted(() => {
  refresh_interval_handle.value = setInterval(update_topic_history, 2000)
})
onUnmounted(() => {
  if (refresh_interval_handle.value !== null) {
    clearInterval(refresh_interval_handle.value)
  }
})

</script>
<template>
  <div>
    <form class="flex px-4 pt-4 pb-2" @submit.prevent="update_topic_history">
      <v-text-field style="width: calc(100% - 12rem);" @keyup.enter="update_topic_history" hide-details
        placeholder="Type the topic or hash(start with #) to track" v-model.lazy="topic_to_track" />
      <v-select class="h-12 mx-1 border rounded-md shadow-sm" label="Hasher" :items="hash_type_items" hide-details
        v-model="hash_type" item-title="name" item-value="value">
      </v-select>
      <v-btn class="aspect-square h-12 cursor-pointer" type="submit" height="3.5rem">
        <span class="mdi-email-arrow-left-outline mdi text-center text-2xl"></span>
        <v-tooltip activator="parent" location="bottom" open-on-hover open-delay="2000">
          Track
        </v-tooltip>
      </v-btn>
    </form>
    <ul style="height: calc(100vh - 16rem);" class="message-list">
      <li v-for="msg in message_history" class="rounded-md">
        <div v-if="msg.Local">
          <p>From: Self</p>
          <p>Message: {{ utf8Decoder.decode(msg.Local) }}</p>
        </div>
        <div v-else>
          <p>From: {{ msg.Remote!.source }}</p>
          <p>Message: {{ utf8Decoder.decode(msg.Remote!.data) }}</p>
        </div>
      </li>
    </ul>
    <v-divider />
    <section class="h-full w-full">
      <textarea v-model="message" class="resize-none w-full p-4" style="height: calc(100% - 3.5rem)"
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
        <v-btn class="w-[4rem] h-10" @click="send_message">Send</v-btn>
      </section>
    </section>
  </div>
</template>

<style>
.message-list {
  display: flex;
  padding: 1rem;
  flex-direction: column;
  gap: 1rem;
  overflow: auto;
}

.message-list>li {
  background-color: rgb(155, 245, 155);
  width: 100%;
  height: fit-content;
  padding: 0.5rem 1rem;
}
</style>