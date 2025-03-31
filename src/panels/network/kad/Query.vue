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
  <section class="single-input px-2">
    <input v-model="peer_to_query" @keydown.enter.exact.prevent="query" 
    placeholder="Query for a Peer ID"
    />
    <button @click="query">Query</button>
  </section>
  <ul>
    <li v-for="ev in events">
      <p>{{ JSON.stringify(ev) }}</p>
    </li>
  </ul>
</template>
