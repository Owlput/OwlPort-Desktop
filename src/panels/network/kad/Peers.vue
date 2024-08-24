<script setup>
import { ref, computed, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api";
import { writeText } from "@tauri-apps/api/clipboard";
import PeerSearchBar from "../../../components/PeerSearchBar.vue";
import AddressBookEntry from "./AddressBookEntry.vue";

const entries = ref([]);
const search_keyword = ref("");
const filtered = computed(() => {
  if (search_keyword === "") return entries.value;
  else return entries.value.filter((v) => v[0].includes(search_keyword.value));
});
function update_kad_entries() {
  invoke("plugin:owlnest-kad|get_all_records").then((v) => {
    entries.value = v;
  });
}
update_kad_entries();
</script>
<template>
  <div class="flex justify-evenly items-center w-full h-8 mt-4">
    <PeerSearchBar place-holder="Type Peer ID and press Enter to search" :refresh="update_kad_entries" v-model="search_keyword"/>
  </div>
  <ul class="overflow-auto px-[6%] pt-2">
    <li v-if="filtered.length === 0">
      Empty address book
    </li>
    <li v-for="relay_info in filtered" class="mt-2">
      <AddressBookEntry :peer-id="relay_info[0]" :addresses="relay_info[1]"/>
    </li>
  </ul>
</template>
