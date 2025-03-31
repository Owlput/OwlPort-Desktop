<script setup lang="ts">
import SearchDisplay from "../../../components/SearchDisplay.vue";
import { invoke } from "@tauri-apps/api/core";
import { TopicRecord } from "./types";
import Topic from "./Topic.vue";

function refresh_topics(): Promise<Array<TopicRecord>> {
  return invoke<Array<TopicRecord>>("plugin:owlnest-gossipsub|get_all_topics").then((v) => { console.log(v); return v.map(v => new TopicRecord(v)) })
}
function filter_topics(source: TopicRecord, search_text: string): boolean {
  if (search_text.startsWith('#'))
    return source.get_hash()?.includes(search_text.slice(1)) as boolean
  return source.get_string()?.includes(search_text) as boolean
}

</script>

<template>
  <SearchDisplay :criteria="filter_topics" :get-or-refresh="refresh_topics" v-slot="slotProps"
    place-holder="Search for topic or topic hash(start with #) here." :min-item-size="108"
    scroller-height-expr="100vh - 6.5rem">
    <div class="mx-8 py-1">
      <div class="border">
        <Topic :topic="slotProps.item" />
      </div>
    </div>
  </SearchDisplay>
</template>
