<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import ListenEvenListener from "./ListenEvenListener.vue";

let listen_addr = ref("/ip4/127.0.0.1/tcp/0");
function listen_on() {
  invoke("plugin:swarm|listen", {
    listenOptions: { addr: listen_addr.value },
  }).then((res) => {
    console.log(res);
  }).catch((e)=>dispatchEvent(new CustomEvent("swarm-listen-failed",{detail:e})));
}
</script>

<template>
  <section class="p-8 shadow-md m-8 rounded-sm">
    <p class="text-left w-full my-2">Listen on an address</p>
    <div class="flex w-full">
      <input
        class="h-12 w-[70%] p-2 text-xl"
        v-model="listen_addr"
        @submit="listen_on"
      />
      <button @click="listen_on" class="mx-4 h-12 w-16">Listen</button>
    </div>
  </section>
  <section>
    <Suspense><ListenEvenListener /></Suspense>
  </section>
</template>
