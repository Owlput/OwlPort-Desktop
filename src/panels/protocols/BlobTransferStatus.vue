<script setup>
import { invoke } from "@tauri-apps/api";
import { onDeactivated, ref } from "vue";
let expand = ref(false);
let connected_peers = ref({});
invoke("plugin:owlnest-blob-transfer|list_connected").then(
  (peers) => (connected_peers.value = peers)
);
onDeactivated(() => {
  expand.value = false;
});
function toggleExpand() {
  if (!expand.value)
    invoke("plugin:owlnest-blob-transfer|list_connected").then(
      (peers) => (connected_peers.value = peers)
    );
  setTimeout(() => (expand.value = !expand.value), 5);
}
</script>
<template>
  <section>
    <section
      class="flex flex-row justify-between items-center border px-4 py-2"
    >
      <p class="hover:cursor-pointer" @click="()=>$router.push('/protocols/blob-transfer')">File Transfer</p>
      <p>Number of reachable peers: {{ connected_peers.length }}</p>
      <div @click="toggleExpand" class="w-6 h-6">
        <span
          class="material-icons"
          style="transform: rotate(90deg)"
          v-if="expand"
        >
          chevron_right
        </span>
        <span v-else class="material-icons"> chevron_right </span>
      </div>
    </section>
    <ul v-if="expand" class="mx-2 border-x border-b">
      <li v-if="connected_peers.length < 1" class="p-2">
        No peer supports this protocol
      </li>
      <li
        v-for="peer in connected_peers"
        class="p-2"
        @click="() => $router.push(`/protocols/blob-transfer/${peer}`)"
      >
        {{ peer }}
      </li>
    </ul>
  </section>
</template>
