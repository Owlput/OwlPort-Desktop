<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import DialEventListener from "./DialEventListener.vue";
import { useRoute } from "vue-router";

let route = useRoute();
defineOptions({
  name: "Dial",
});
let peer_to_dial = ref(route.query?.dial ? route.query.dial : "");
function dial() {
  if (!peer_to_dial.value) {
    return;
  }
  invoke("plugin:owlnest-swarm|dial", {
    dialOptions: { address: peer_to_dial.value },
  }).catch((e) =>
    dispatchEvent(new CustomEvent("swarm-dial-failed", { detail: e }))
  );
}
</script>

<template>
  <section class="px-8 py-4 border-b">
    <p class="text-left w-full px-4 text-lg">Dial a peer</p>
    <div class="single-input">
      <input
        class="text-xl"
        v-model="peer_to_dial"
        @keypress.enter.exact.prevent="() => dial()"
      />
      <button @click="dial">Dial</button>
    </div>
  </section>
  <section style="height: calc(100vh - 9.25rem - 1px)">
    <p class="px-8 py-2 text-lg">Events:</p>
    <DialEventListener />
  </section>
</template>
