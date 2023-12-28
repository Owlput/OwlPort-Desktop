<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import ConnectedPeerEntry from "./ConnectedPeerEntry.vue";

const connectedPeers = ref([]);
invoke("plugin:owlnest-swarm|list_connected").then((list) => {
  connectedPeers.value = list;
});
</script>

<template>
  <div class="wrapper flex flex-col">
    <ul class="event-list select-none overflow-auto px-8 py-4" style="height: calc(100vh - 3rem);">
      <li v-if="connectedPeers.length === 0" class="text-center">
        No peers connected.
      </li>
      <li v-for="peer in connectedPeers"  class="bg-green-200">
        <Suspense>
          <ConnectedPeerEntry :peer-id="peer" />
        </Suspense>
      </li>
    </ul>
  </div>
</template>
