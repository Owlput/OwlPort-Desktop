<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import RelayEntry from "./RelayEntry.vue";

const relay_list = ref({});
function update_relay_list() {
  invoke("plugin:owlnest-relay|list_relays").then((v) => {
    relay_list.value = v;
  });
}
update_relay_list();
</script>
<template>
  <div class="px-4 flex justify-between items-center w-full">
    <p>Connected relays:</p>
    <button class="h-[24px]" @click="update_relay_list">
      <span class="material-icons">refresh</span>
    </button>
  </div>
  <ul
    class="event-list text-autowrap select-none"
    style="height: calc(100vh - 4rem)"
  >
    <li v-if="relay_list.length < 1">
      <p class="text-center">No relay connected.</p>
    </li>
    <li v-for="relay in relay_list" class="rounded-sm justify-between">
      <Suspense> <RelayEntry :peer-id="relay" /></Suspense>
    </li>
  </ul>
</template>
