<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

const listen_addr = ref("/ip4/0.0.0.0/tcp/0");
const active_listeners = ref([]);
function update_listener_list() {
  invoke("plugin:owlnest-swarm|list_listeners").then((result) => {
    active_listeners.value = result;
  });
}
update_listener_list();
function listen_on() {
  invoke("plugin:owlnest-swarm|listen", {
    listenOptions: { addr: listen_addr.value },
  }).catch((e) =>
    dispatchEvent(new CustomEvent("swarm-listen-failed", { detail: e }))
  );
  setTimeout(update_listener_list, 100);
}
</script>

<template>
  <section class="px-8 py-4 border-b">
    <p class="text-left w-full px-4 text-lg select-none">
      Listen on an address
    </p>
    <div class="single-input">
      <input class="text-xl" v-model="listen_addr" @submit="listen_on" />
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

    <ul style="height: calc(100vh - 12.5rem - 1px); overflow: auto" class="event-list select-none">
      <li v-if="active_listeners.length < 1"><p>No active listeners.</p></li>
      <li
        v-for="addr in active_listeners"
        class="my-1 shadow-md rounded-sm w-full p-2 bg-green-100 text-autowrap"
        @dblclick="writeText(addr)"
      >
        <p>{{ addr }}</p>
      </li>
    </ul>
  </section>
</template>
