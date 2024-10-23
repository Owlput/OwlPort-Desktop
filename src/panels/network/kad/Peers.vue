<script setup lang="ts">
import { ref, computed, Ref } from "vue";
import { invoke } from "@tauri-apps/api";
import SearchBar from "../../../components/SearchBar.vue";
import AddressBookEntry from "./AddressBookEntry.vue";
import { isBodylessHandler } from "../../../utils";
import { PeerStub } from "./types";

const entries: Ref<Array<PeerStub>> = ref([]);
const search_keyword = ref("");
const filtered_list = computed(() => {
  if (search_keyword.value === "") return entries.value;
  else return entries.value.filter((v) => v.peer_id.includes(search_keyword.value));
});
function update_kad_entries() {
  invoke<Array<PeerStub>>("plugin:owlnest-kad|get_all_records")
    .then((v) => {
      entries.value = v;
    })
    .catch(isBodylessHandler);
}
update_kad_entries();
</script>
<template>
  <div class="flex justify-evenly items-center w-full">
    <SearchBar place-holder="Type Peer ID and press Enter to search" :refresh="update_kad_entries"
      v-model="search_keyword" />
  </div>
  <li v-if="filtered_list.length === 0">Empty address book</li>
  <section v-else style="height: calc(100vh - 6.5rem)" class=" overflow-auto">
    <DynamicScroller :items="filtered_list" :min-item-size="10" :buffer="15" key-field="peer_id" class="h-full">
      <template v-slot="{ item, index, active }">
        <DynamicScrollerItem :item="item" :active="active" :data-index="index">
          <AddressBookEntry :peer="item" />
        </DynamicScrollerItem>
      </template>
    </DynamicScroller>
  </section>
</template>
