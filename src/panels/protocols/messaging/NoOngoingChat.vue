<script setup>
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
let connected_peers = ref({});
invoke("plugin:owlnest-messaging|setup").then(
  (peers) => (connected_peers.value = peers)
);
</script>
<template>
  <section>
    <p>No ongoing chat.</p>
    <p>Below are some peers you may start a chat with</p>
    <ul class="mx-2 border-x border-b">
      <li v-if="Object.keys(connected_peers).length < 1" class="p-2">
        No peer supports this protocol
      </li>
      <li
        v-for="peer in Object.keys(connected_peers)"
        class="p-2"
        @click="() => $router.push(`/protocols/messaging/${peer}`)"
      >
        {{ peer }}
      </li>
    </ul>
  </section>
</template>
