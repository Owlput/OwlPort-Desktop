<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onUnmounted, ref } from "vue";
import ConnectedPeerEntry from "./ConnectedPeerEntry.vue";
import { listen } from "@tauri-apps/api/event";

const connected_peers = ref(new Set());
const listener_handle = ref(() => {});
function handle_swarm_event(ev) {
  if (ev.payload?.ConnectionClosed) {
    if (ev.payload.ConnectionClosed.num_established < 1) {
      connected_peers.value.delete(ev.payload.ConnectionClosed.peer_id);
    }
    return;
  }
  if (ev.payload?.ConnectionEstablished) {
    connected_peers.value.add(ev.payload.ConnectionEstablished.peer_id);
  }
}
invoke("plugin:owlnest-swarm|list_connected").then((list) => {
  list.forEach((element) => {
    connected_peers.value.add(element);
  });
});
listen("swarm-emit", (ev) => {
  handle_swarm_event(ev);
}).then((handle) => (listener_handle.value = handle));
onUnmounted(() => {
  listener_handle.value();
});
</script>

<template>
  <div class="wrapper flex flex-col">
    <ul
      class="event-list select-none overflow-auto px-8 py-4"
      style="height: calc(100vh - 3rem)"
    >
      <li v-if="connected_peers.size === 0" class="text-center">
        No peers connected.
      </li>
      <li v-for="peer in connected_peers" class="bg-green-200">
        <Suspense>
          <ConnectedPeerEntry :peer-id="peer" />
        </Suspense>
      </li>
    </ul>
  </div>
</template>
