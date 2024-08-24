<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import RelayEntry from "./RelayEntry.vue";
import PeerSearchBar from "../../components/PeerSearchBar.vue";

const relay_list = ref(null);
const search_keyword = ref("");

function update_relay_list() {
  invoke("plugin:owlnest-relay|list_relays").then((v) => {
    v.sort((a, b) => {
      if (a[1] > 0) return a[1] - b[1];
      else return 1;
    });
    relay_list.value = v;
  });
}
let filtered_list = computed(() => {
  if (search_keyword.value !== "")
    return relay_list.value.filter((v) => v[0].includes(search_keyword.value));
  else return relay_list.value;
});
update_relay_list();
</script>
<template>
  <div class="flex justify-evenly items-center pt-4">
    <PeerSearchBar place-holder="Type Peer ID and press Enter to search" :refresh="update_relay_list" v-model="search_keyword"/>
  </div>
  <ul
    class="event-list text-autowrap select-none overflow-auto px-8 py-4"
    style="height: calc(100vh - 5.5rem)"
  >
    <li v-if="relay_list?.length === 0">
      <p class="text-center">No relay connected.</p>
    </li>
    <li v-if="relay_list === null">
      <p class="text-center">Fetching data from backend...</p>
    </li>
    <li v-for="relay in filtered_list" class="rounded-sm justify-between">
      <Suspense>
        <RelayEntry :peer-id="relay[0]" :latency="relay[1]"
      /></Suspense>
    </li>
  </ul>
</template>
