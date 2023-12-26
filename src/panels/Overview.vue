<script setup>
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const showConnectedPeers = ref(false);
const showTrustedPeers = ref(false);
const nestState = {
  status: "up",
  address: "127.0.0.1:20001",
  latency: "1",
};

const trustedPeers = [
  {
    peerId: "test",
    state: "up",
  },
  {
    peerId: "test",
    state: "unstable",
  },
  {
    peerId: "test",
    state: "not-connected",
  },
];
function nestStateSwitch(state) {
  switch (state) {
    case "up":
      return "#80ED99";
    case "not-connected":
      return "#DEE2E6";
    case "unstable":
      return "#FFD60A";
  }
}
</script>
<template>
  <div class="flex flex-wrap min-h-screen">
    <div class="wrapper bg-slate-200 flex flex-col">
      <button
        @click="
          () => {
            showTrustedPeers = !showTrustedPeers;
          }
        "
      >
        Trusted Peers
      </button>
      <ol v-if="showTrustedPeers">
        <li class="grid grid-cols-2">
          <p>Peer ID</p>
          <p>Operations</p>
        </li>
        <ul>
          <li v-for="peer in trustedPeers" class="grid grid-cols-2 p-1">
            <button
              class="navBtn hover:ring-1 active:brightness-[0.9]"
              :style="`background-color:${nestStateSwitch(peer.state)}`"
            >
              {{ peer.peerId }}
            </button>
            <div class="flex flex-row justify-evenly w-full">
              <button v-if="peer.state == 'not-connected'">Dial</button>
              <button v-if="peer.state != 'not-connected'">Disconnect</button>
            </div>
          </li>
        </ul>
      </ol>
    </div>
  </div>
</template>
<style scoped>
.wrapper {
  margin: 0.5rem;
  padding: 0.25rem;
  height: fit-content;
  width: fit-content;
}
</style>
