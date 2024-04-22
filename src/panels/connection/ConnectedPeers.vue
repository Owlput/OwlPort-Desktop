<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onUnmounted, ref, computed } from "vue";
import ConnectedPeerEntry from "./ConnectedPeerEntry.vue";

const connected_peers = ref(null);
const search = ref("");
const filtered_list = computed(()=>{
  if (search.value !== "")
  return connected_peers.value.values().filter((v)=>v.includes(search.value));else return connected_peers.value 
});

function update_list(){
  invoke("plugin:owlnest-swarm|list_connected").then((list) => {
    connected_peers.value = list
}).catch((e)=>{
  console.error(e);
  connected_peers.value = undefined
});
}
update_list()
const interval_id = setInterval(()=>update_list(),5000);
onUnmounted(()=>{
  clearInterval(interval_id)
})
</script>

<template>
  <div class="wrapper flex flex-col">
    <div class="flex justify-evenly items-center w-full h-8 mt-4">
      <input
        class="h-8 w-[80%] px-4"
        placeholder="Type PeerID and press enter to search"
        v-model.lazy="search"
      />
      <button class="h-8 w-8" @click="update_list">
        <span class="material-icons h-full w-full">refresh</span>
      </button>
    </div>
    <ul
      class="event-list select-none overflow-auto px-8 py-4"
      style="height: calc(100vh - 5.5rem)"
    >
      <li
        v-if="connected_peers?.size === 0"
        class="text-center"
      >
        No peers connected.
      </li>
      <li v-if="connected_peers === null">Fetching data from backend...</li>
      <li v-for="peer in filtered_list" class="bg-green-200">
        <Suspense>
          <ConnectedPeerEntry :peer-id="peer" />
        </Suspense>
      </li>
    </ul>
  </div>
</template>
