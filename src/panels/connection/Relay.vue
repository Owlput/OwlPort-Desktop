<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import RelayEntry from "./RelayEntry.vue";
import { Status } from "../../utils";
import { RelayStub } from "./types";
import SearchDisplay from "../../components/SearchDisplay.vue";

const status = ref(Status.PendingInit);

function update_relay_list() {
  return invoke<Array<RelayStub>>("plugin:owlnest-relay|list_relays")
    .then((v) => {
      return v
    })
}
update_relay_list();
function filter(source: RelayStub, search_text: String): boolean {
  return source.peer_id.includes(search_text.valueOf())
}
</script>
<template>
  <SearchDisplay :criteria="filter" :get-or-refresh="update_relay_list" v-slot="slotProps"
    place-holder="Search for topic or topic hash(start with #) here." :min-item-size="36"
    scroller-height-expr="100vh - 6.5rem" @load-result="(result) => { status = result }">
    <RelayEntry :relay_stub="slotProps.item"></RelayEntry>
  </SearchDisplay>
</template>
