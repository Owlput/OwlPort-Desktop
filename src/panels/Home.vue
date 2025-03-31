<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
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
  <section class="p-4 bg-green-300">
    <p v-if="local_peer_id === ''">Backend Not Connected</p>
    <div v-else-if="local_peer_id" class="flex justify-evenly items-center flex-wrap">
      <p class="max-w-[100%] text-autowrap select-none">Peer {{ local_peer_id }}</p> 
      <button class="aspect-square w-fit" @click="() => writeText(local_peer_id)"><span
          class="material-icons icon-center">content_copy</span></button>
    </div>
    <p v-else>Fetching Peer ID from backend...</p>
  </section>
</template>
