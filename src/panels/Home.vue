<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
let local_peer_id = ref("");
invoke("plugin:owlnest-swarm|get_local_peer_id").then((res) => {
  local_peer_id.value = res;
});
</script>

<template>
  <section class="p-4 bg-green-300">
    <p class="text-center">Peer {{ local_peer_id }}</p>
  </section>
  <button @click="() => invoke('plugin:popup-tester|emit_test_event')">
    Test popup
  </button>
  <button @click="() => invoke('plugin:owlnest-messaging|init_subapp')">
    init subwindow
  </button>
</template>
