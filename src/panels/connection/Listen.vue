<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { isBodylessHandler } from "../../utils";
import EntryExpand from "../../components/EntryExpand.vue";

const listen_addr = ref("/ip4/0.0.0.0/tcp/0");
const active_listeners: Ref<Array<String>> = ref([]);
function update_listener_list() {
  invoke<Array<String>>("plugin:owlnest-swarm|list_listeners")
    .then((result) => {
      active_listeners.value = result;
    })
    .catch(isBodylessHandler);
}
update_listener_list();
function listen_on() {
  invoke("plugin:owlnest-swarm|listen", {
    listenOptions: { addr: listen_addr.value },
  }).catch(isBodylessHandler);
  setTimeout(update_listener_list, 100);
}
</script>

<template>
  <section class="px-8 py-4 border-b">
    <p class="text-left w-full px-4 text-lg select-none">
      Listen on an address
    </p>
    <div class="single-input">
      <input class="text-xl" v-model="listen_addr" @keypress.enter.exact.prevent="() => listen_on()" />
      <button @click="listen_on">Listen</button>
    </div>
  </section>
  <section>
    <div class="flex justify-between items-center select-none">
      <p class="text-lg px-8 py-2 w-fit">Active Listeners:</p>
      <button class="float-right h-[26px] mr-8" @click="update_listener_list">
        <span class="material-icons">refresh</span>
      </button>
    </div>
    <p v-if="active_listeners.length < 1">No active listeners.</p>
    <ul v-else style="height: calc(100vh - 12.5rem - 1px); overflow: auto" class="select-none px-8">
      <li v-for="addr in active_listeners" class="my-1 shadow-md w-full h-10 p-0">
        <EntryExpand>
          <template #main>
            <p class="w-full h-full p-2 bg-green-100">{{ addr }}</p>
          </template>
          <template #more>
            <div class="flex gap-1 bg-green-100 h-full p-1">
              <button><span class="material-icons icon-center">delete_forever</span></button>
              <button class="" @click="writeText(addr.toString())">
                <span class="material-icons icon-center">content_copy</span></button>
            </div>
          </template>
        </EntryExpand>
      </li>
    </ul>
  </section>
</template>