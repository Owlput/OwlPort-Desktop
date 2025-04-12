<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, Ref } from "vue";
import ConnectedPeerEntry from "./ConnectedPeerEntry.vue";
import { Status } from "../../utils";
import SearchDisplay from "../../components/SearchDisplay.vue";

const status: Ref<Status> = ref(Status.PendingInit)

function update_list() {
  return invoke<Array<String>>("plugin:owlnest-swarm|list_connected")
    .then((list) => {
      status.value = Status.OK;
      return list;
    })
}
function filter(source: String, search_text: String): boolean {
  return source.includes(search_text.valueOf())
}
</script>

<template>
  <p v-if="status === Status.NoBackend">
    Bodyless mode.
  </p>
  <div v-else class="w-full px-4 pt-4">
    <SearchDisplay place-holder="Type Peer ID and press Enter to search" :get-or-refresh="update_list"
      :criteria="filter" v-slot="slotProps" :min-item-size="64" scroller-height-expr="100vh - 8.5rem">
      <ConnectedPeerEntry :peer-id="slotProps.item" />
    </SearchDisplay>
  </div>
</template>
