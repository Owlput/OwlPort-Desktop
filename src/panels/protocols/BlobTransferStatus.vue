<script setup>
import { invoke } from "@tauri-apps/api";
import { onUnmounted, ref } from "vue";
let expand = ref(false);
let connected_peers = ref([]);
invoke("plugin:owlnest-blob-transfer|list_connected").then(
  (peers) => (connected_peers.value = peers)
);
function toggleExpand() {
  expand.value = !expand.value;
}
function spawn_window(peer) {
  invoke("plugin:owlnest-blob-transfer|spawn_window", { peer });
}
function update_list() {
  invoke("plugin:owlnest-blob-transfer|list_connected").then(
    (peers) => (connected_peers.value = peers)
  );
}
const interval_id = setInterval(update_list, 5000);
onUnmounted(() => {
  clearInterval(interval_id);
});
</script>
<template>
  <section>
    <section
      class="flex flex-row justify-between items-center border px-4 py-2"
    >
      <p class="w-[20%] text-center sm:text-left">File Transfer</p>
      <p>Number of reachable peers: {{ connected_peers.length }}</p>
      <div @click="toggleExpand" class="w-6 h-6 hover:cursor-pointer">
        <span
          class="material-icons float-right"
          style="transform: rotate(90deg)"
          v-if="expand"
        >
          chevron_right
        </span>
        <span v-else class="material-icons w-[20%]"> chevron_right </span>
      </div>
    </section>
    <ul v-if="expand" class="mx-2 border-x border-b">
      <li v-if="connected_peers.length < 1" class="p-2">
        No peer supports this protocol
      </li>
      <li
        v-for="peer in connected_peers"
        class="p-2"
        @click="() => spawn_window(peer)"
      >
        <p class="font-mono">{{ peer }}</p>
      </li>
    </ul>
  </section>
</template>
