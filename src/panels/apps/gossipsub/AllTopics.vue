<script setup lang="ts">
import SearchDisplay from "../../../components/SearchDisplay.vue";
import { invoke } from "@tauri-apps/api";
import { ReadableTopic } from "./types";


function refresh_topics(): Promise<Array<ReadableTopic>> {
  return invoke<Array<ReadableTopic>>("plugin:libp2p-gossipsub|get_all_topics")
}
function filter_topics(source: Array<ReadableTopic>, search_text: String): Array<ReadableTopic> {
  if (search_text.length === 0) return source;
  if (search_text.startsWith('#'))
    return source.filter((v) => v.get_hash()?.includes(search_text.slice(1)));
  return source.filter((v) => v.get_string()?.includes(search_text.valueOf()));
}

</script>

<template>
  <SearchDisplay :criteria="filter_topics" :get_or_refresh="refresh_topics" v-slot="slotProps">
    <p>Topic hash: {{ slotProps.item }}</p>
  </SearchDisplay>
</template>
