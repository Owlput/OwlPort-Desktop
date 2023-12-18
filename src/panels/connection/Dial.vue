<script setup>
import { onActivated, ref } from "vue";
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

async function dial() {
  invoke("plugin:swarm|dial", {
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
  <section class="p-8 shadow-md m-8 rounded-sm">
    <p class="text-left w-full my-2">Dial a peer</p>
    <div class="flex w-full">
      <input
        class="h-12 w-[70%] p-2 text-xl"
        v-model="peer_to_dial"
        @submit="dial"
      />
      <button @click="dial" class="mx-4 h-12 w-16">Dial</button>
    </div>
  </section>
  <section>
    <Suspense>
      <DialEventListener />
    </Suspense>
  </section>
</template>
