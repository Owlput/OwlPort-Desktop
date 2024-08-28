<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onUnmounted, ref, computed } from "vue";
import ConnectedPeerEntry from "./ConnectedPeerEntry.vue";
import PeerSearchBar from "../../components/PeerSearchBar.vue";
import {isBodylessHandler} from "../../utils"

const connected_peers = ref(null);
const search_keyword = ref("");
const filtered_list = computed(() => {
  if (search_keyword.value !== "")
    return connected_peers.value
      .values()
      .filter((v) => v.includes(search_keyword.value));
  else return connected_peers.value;
});

function update_list() {
  invoke("plugin:owlnest-swarm|list_connected")
    .then((list) => {
      connected_peers.value = list;
    })
    .catch(e=>isBodylessHandler(e)?connected_peers.value = undefined:null);
}
update_list();
const interval_id = setInterval(() => update_list(), 5000);
onUnmounted(() => {
  clearInterval(interval_id);
});
</script>

<template>
  <div class="wrapper flex flex-col">
    <div class="flex justify-evenly items-center w-full h-8 mt-4">
      <PeerSearchBar place-holder="Type Peer ID and press Enter to search" :refresh="update_list" v-model="search_keyword"/>
    </div>
    <ul
      class="event-list select-none overflow-auto px-8 py-4"
      style="height: calc(100vh - 5.5rem)"
    >
      <li v-if="connected_peers?.length === 0" class="text-center">
        No peers connected.
      </li>
      <li v-if="connected_peers === null" class="text-center">
        Fetching data from backend...
      </li>
      <li v-for="peer in filtered_list" class="bg-green-200">
        <Suspense>
          <ConnectedPeerEntry :peer-id="peer" />
        </Suspense>
      </li>
    </ul>
  </div>
</template>
