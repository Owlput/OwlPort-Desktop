<script setup lang="ts">
import SearchDisplay from "../../../components/SearchDisplay.vue";
import { invoke } from "@tauri-apps/api";
import { ReadableTopic } from "./types";
import Topic from "./Topic.vue";

function refresh_topics(): Promise<Array<ReadableTopic>> {
  return invoke<Array<ReadableTopic>>("plugin:libp2p-gossipsub|get_all_topics").then((v) => v.map(v => new ReadableTopic(v)))
}
function filter_topics(source: ReadableTopic, search_text: String): boolean {
  if (search_text.startsWith('#'))
    return source.get_hash()?.includes(search_text.slice(1)) as boolean
  return source.get_string()?.includes(search_text.valueOf()) as boolean
}

</script>

<template>
  <SearchDisplay :criteria="filter_topics" :get-or-refresh="refresh_topics" v-slot="slotProps"
    place-holder="Search for topic or topic hash(start with #) here." :min-item-size="36"
    scroller-height-expr="100vh - 6.5rem">
    <Topic :topic="slotProps.item" />
  </SearchDisplay>
</template>
