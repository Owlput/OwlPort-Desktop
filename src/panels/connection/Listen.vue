<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const listen_addr = ref("/ip4/0.0.0.0/tcp/0");
const active_listeners = ref([]);
function update_listener_list(){
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
  setTimeout(update_listener_list,100)
}
</script>

<template>
  <section class="px-8 py-4 border-b">
    <p class="text-left w-full px-4 text-lg">Listen on an address</p>
    <div class="single-input">
      <input class="text-xl" v-model="listen_addr" @submit="listen_on" />
      <button @click="listen_on">Listen</button>
    </div>
  </section>
  <section class="px-8 py-4">
    <div class="flex justify-between">
      <p class="text-lg px-4 w-fit">Active Listeners:</p>
      <button class="float-right h-[26px]" @click="update_listener_list"><span class="material-icons">refresh</span></button>
    </div>
    <ul>
      <li
        v-for="addr in active_listeners"
        class="my-1 shadow-md rounded-sm w-full p-2 bg-green-100"
      >
        <p>{{ addr }}</p>
      </li>
    </ul>
  </section>
</template>
