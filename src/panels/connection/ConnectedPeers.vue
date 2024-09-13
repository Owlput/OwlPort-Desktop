<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onUnmounted, ref, Ref, computed } from "vue";
import ConnectedPeerEntry from "./ConnectedPeerEntry.vue";
import SearchBar from "../../components/SearchBar.vue";
import { isBodylessHandler, Status } from "../../utils";

const connected_peers: Ref<Array<String>> = ref([]);
const status: Ref<Status> = ref(Status.PendingInit)
const search_keyword: Ref<String> = ref("");
const filtered_list = computed(() => {
  if (search_keyword.value === "") return connected_peers.value;
  return connected_peers.value
    .filter((v) => v.includes(search_keyword.value.valueOf()));

});

function update_list() {
  invoke<Array<String>>("plugin:owlnest-swarm|list_connected")
    .then((list) => {
      status.value = Status.OK;
      connected_peers.value = list;
    })
    .catch((e) => {
      if (isBodylessHandler(e)) {
        status.value = Status.NoBackend
      }
    });
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
      <SearchBar place-holder="Type Peer ID and press Enter to search" :refresh="update_list"
        v-model="search_keyword" />
    </div>
    <ul class="event-list select-none overflow-auto px-8 py-4" style="height: calc(100vh - 5.5rem)">
      <li v-if="status === Status.NoBackend">
        Bodyless mode.
      </li>
      <li v-else-if="connected_peers.length === 0" class="text-center">
        No peers connected.
      </li>
      <li v-for="peer in filtered_list" class="bg-green-200">
        <Suspense>
          <ConnectedPeerEntry :peer-id="peer.valueOf()" />
        </Suspense>
      </li>
    </ul>
  </div>
</template>
