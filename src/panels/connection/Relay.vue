<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import RelayEntry from "./RelayEntry.vue";

const relay_list = ref([]);
const search = ref("");

function update_relay_list() {
  invoke("plugin:owlnest-relay|list_relays").then((v) => {
    v.sort((a,b) => {if (a[1] > 0) return a[1] - b[1]; else return 1});
    relay_list.value = v;
  });
}
let filtered_list = computed(()=>{
  if (search.value !== "")
  return relay_list.value.filter((v)=>v[0].includes(search.value));else return relay_list.value 
})
update_relay_list();
</script>
<template>
  <div class="px-4 flex justify-between items-center flex-col w-full">
    <p class="text-xl"> Connected relays</p>
    <div class="flex justify-evenly items-center w-[80%] h-8">
      <input class="h-8 w-[80%] px-4" placeholder="Type PeerID and press enter to search" v-model.lazy="search"/>
    <button class="h-8 w-8" @click="update_relay_list">
      <span class="material-icons h-full w-full">refresh</span>
    </button>
  </div>
  </div>
  <ul
    class="event-list text-autowrap select-none"
    style="height: calc(100vh - 6.25rem)"
  >
    <li v-if="relay_list.length < 1">
      <p class="text-center">No relay connected.</p>
    </li>
    <li v-for="relay in filtered_list" class="rounded-sm justify-between">
      <Suspense> <RelayEntry :peer-id="relay[0]" :latency="relay[1]" /></Suspense>
    </li>
  </ul>
</template>
