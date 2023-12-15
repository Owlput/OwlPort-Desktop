<script setup>
import { onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import SwarmEventListener from "./SwarmEventListener.vue";
defineOptions({
  name:"Dial"
})
let peer_to_dial = ref("/ip4/127.0.0.1/tcp/10086");
console.log("Dial created")

async function dial() {
  invoke("plugin:swarm|dial", {
    dialOptions: { address: peer_to_dial.value },
  }).then((v) => {});
}
onUnmounted(() => {
  console.log("Dial destroyed")
});
</script>

<template>
  <section class="p-8 shadow-md m-8 rounded-sm">
    <p class="text-left w-full my-2">Dial a peer</p>
    <div class="flex w-full">
      <input class="h-12 w-[70%] p-2 text-xl" v-model="peer_to_dial" />
      <button @click="dial" class="mx-4 h-12 w-16">Dial</button>
    </div>
  </section>
  <section>
    <Suspense>
      <SwarmEventListener />
    </Suspense>
  </section>
</template>
