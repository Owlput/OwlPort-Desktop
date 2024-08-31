<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { isBodylessHandler } from "../utils";
const local_peer_id = ref("null123");
invoke("plugin:owlnest-swarm|get_local_peer_id")
  .then((res) => {
    local_peer_id.value = res;
    localStorage.setItem("local_peer_id", res);
  })
  .catch((e) => {
    if (isBodylessHandler(e)) {
      local_peer_id.value = "";
      localStorage.setItem("local_peer_id", "");
    }
  });
</script>

<template>
  <section class="p-4 bg-green-300 text-center">
    <p v-if="local_peer_id === ''">Backend Not Connected</p>
    <p v-else-if="local_peer_id">Peer {{ local_peer_id }}</p>
    <p v-else>Fetching Peer ID from backend...</p>
  </section>
</template>
