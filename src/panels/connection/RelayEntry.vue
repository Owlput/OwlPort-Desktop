<script setup>
import { invoke } from "@tauri-apps/api";
import { ref, watch } from "vue";
const props = defineProps({
  peerId: String,
});
const relay_info = ref({});
await invoke("plugin:owlnest-relay|get_relay_status", {
  relay: props.peerId,
}).then((v) => {
  v.address = v.address.filter(
    (v) => !(v.includes("127.0.0.1") || v.includes("wss"))
  );
  relay_info.value = v;
});

const show_listenable = ref(false);
const show_listened = ref(false);

watch([show_listenable, show_listened], () => {
  invoke("plugin:owlnest-relay|get_relay_status", {
    relay: props.peerId,
  }).then((v) => {
    v.address = v.address.filter(
      (v) => !(v.includes("127.0.0.1") || v.includes("wss"))
    );
    relay_info.value = v;
  });
});

function listen_on_relay(addr, peer_id) {
  invoke("plugin:owlnest-swarm|listen", {
    listenOptions: {
      addr: `${addr}/p2p/${peer_id}/p2p-circuit`,
    },
  }).catch((e) => console.log(e));
  setTimeout(
    () =>
      invoke("plugin:owlnest-relay|get_relay_status", {
        relay: props.peerId,
      }).then((v) => {
        v.address = v.address.filter(
          (v) => !(v.includes("127.0.0.1") || v.includes("wss"))
        );
        relay_info.value = v;
      }),
    50
  );
}
</script>
<template>
  <div class="flex justify-between flex-nowrap border px-2">
    <p class="select-text">{{ props.peerId }}</p>
    <p>RTT: {{ relay_info.latency }}ms</p>
  </div>

  <section class="mx-2 border-x text-autowrap">
    <p
      @click="() => (show_listenable = !show_listenable)"
      class="hover:cursor-pointer px-2"
    >
      Listenable addresses({{ relay_info.address.length }}):
    </p>
    <ul class="px-4 event-list" v-if="show_listenable">
      <li v-if="relay_info.address.length === 0">
        No listenable address or have all been listened
      </li>
      <li v-for="addr in relay_info.address" class="w-full">
        <p
          @dblclick="() => listen_on_relay(addr, props.peerId)"
          class="hover:cursor-pointer"
        >
          {{ addr }}
        </p>
      </li>
    </ul>
  </section>
  <section class="mx-2 border text-autowrap">
    <p
      @click="show_listened = !show_listened"
      class="hover:cursor-pointer px-2"
    >
      Listened addresses({{ relay_info.listened_address.length }}):
    </p>
    <ul class="event-list px-4" v-if="show_listened">
      <li v-if="relay_info.listened_address.length === 0">
        No address is being listened on.
      </li>
      <li v-for="addr in relay_info.listened_address">
        <p>{{ addr }}</p>
      </li>
    </ul>
  </section>
</template>
