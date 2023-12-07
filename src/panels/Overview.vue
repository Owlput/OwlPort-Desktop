<script setup>
import { ref } from "vue";

const showConnectedPeers = ref(false);
const showTrustedPeers = ref(false);
const nestState = {
  status: "up",
  address: "127.0.0.1:20001",
  latency: "1",
};
const connectedPeers = [
  {
    addr: "1.1.1.1:57354",
    ver: "0.0.1",
    latency: "50",
    potocols: ["ping"],
  },
  {
    addr: "1.1.1.1:57354",
    ver: "0.0.1",
    latency: "50",
    potocols: ["ping"],
  },
  {
    addr: "1.1.1.1:57354",
    ver: "0.0.1",
    latency: "50",
    potocols: ["ping"],
  },
];
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
    <div
      class="wrapper flex flex-col"
      :style="`background-color:${nestStateSwitch(nestState.status)}`"
    >
      <p>Nest Connected</p>
      <p>{{ nestState.address }}</p>
      <p>Latency: {{ nestState.latency }} ms</p>
    </div>
    <div class="wrapper bg-slate-200 flex flex-col">
      <button
        @click="
          () => {
            showConnectedPeers = !showConnectedPeers;
          }
        "
      >
        Connected Peers
      </button>
      <ol v-if="showConnectedPeers">
        <li class="grid grid-cols-4">
          <p>Address</p>
          <p>Version</p>
          <p>Latency</p>
          <p>Supported Potocols</p>
        </li>
        <ul>
          <li v-for="peer in connectedPeers" class="grid grid-cols-4">
            <p>{{ peer.addr }}</p>
            <p>{{ peer.ver }}</p>
            <p>{{ peer.latency }}</p>
            <p>{{ peer.potocols }}</p>
          </li>
        </ul>
      </ol>
    </div>
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
          <li v-for="peer in trustedPeers" class="grid grid-cols-2 m-1 p-1">
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
