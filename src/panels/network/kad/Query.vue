<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { useRoute } from "vue-router";
import { isBodylessHandler } from "../../../utils";

const route = useRoute();
const peer_to_query = ref(route.query?.query ? route.query.query : "");
const events = ref([]);

function query() {
  invoke("plugin:owlnest-kad|lookup", { peerId: peer_to_query.value })
    .then((result) => console.log(result))
    .catch(isBodylessHandler);
}
</script>

<template>
  <form class="single-input mt-4 mx-4" @submit.prevent="query">
    <v-text-field v-model="peer_to_query" @keydown.enter.exact.prevent="query" placeholder="Query for a Peer ID" />
    <v-btn type="submit" size="large" height="3.5rem">Query</v-btn>
  </form>
  <v-divider />
  <ul>
    <li v-for="ev in events">
      <p>{{ JSON.stringify(ev) }}</p>
    </li>
  </ul>
</template>
