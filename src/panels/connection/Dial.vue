<script setup>
import { onActivated, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import DialEventListener from "./DialEventListener.vue";
import { useRoute } from "vue-router";
let route = useRoute();
defineOptions({
  name: "Dial",
});
let peer_to_dial = ref(
  route.query?.dial ? route.query.dial : "/ip4/127.0.0.1/tcp/10086"
);
const tab = ref(0);

async function dial() {
  invoke("plugin:owlnest-swarm|dial", {
    dialOptions: { address: peer_to_dial.value },
  }).catch((e) =>
    dispatchEvent(new CustomEvent("swarm-dial-failed", { detail: e }))
  );
}
onActivated(() => {
  if (route.query?.dial) peer_to_dial.value = route.query.dial;
});
</script>

<template>
  <section class="px-8 py-4 border-b">
    <p class="text-left w-full px-4 text-lg">Dial a peer</p>
    <div class="single-input">
      <input class="text-xl" v-model="peer_to_dial" @submit="dial" />
      <button @click="dial">Dial</button>
    </div>
  </section>
  <section class="py-4">
    <p class="px-12 text-lg">Events:</p>
    <Suspense>
      <DialEventListener />
    </Suspense>
  </section>
</template>
