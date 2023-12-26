<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const connectedPeers = ref({});
invoke("plugin:owlnest-swarm|list_connected").then((list) => {
  connectedPeers.value = list;
});
</script>

<template>
  <div class="wrapper bg-slate-200 flex flex-col">
    <ol>
      <li class="grid grid-cols-3 text-center">
        <p>Peer ID</p>
        <p>App Protocol Set</p>
        <p>Supported Potocols</p>
      </li>
      <ul class="text-autowrap">
        <li
          v-for="peer in Object.keys(connectedPeers)"
          class="grid grid-cols-3 mx-4 my-2 border border-slate-950"
        >
          <p>{{ peer }}</p>
          <p>{{ JSON.stringify(connectedPeers[peer].protocol_version) }}</p>
          <ul>
            <li v-for="protocol in connectedPeers[peer].supported_protocols">
              {{ protocol }}
            </li>
          </ul>
        </li>
      </ul>
    </ol>
  </div>
</template>
