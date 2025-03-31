<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, Ref, onUnmounted, shallowRef } from "vue";
import { isBodylessHandler } from "../../utils";

let expand = ref(false);
let connected_peers: Ref<Array<String>> = shallowRef([]);

function toggleExpand() {
  expand.value = !expand.value;
}
function update_list() {
  invoke<Array<String>>("plugin:owlnest-messaging|list_connected")
    .then((peers) => (connected_peers.value = peers))
    .catch(isBodylessHandler);
}
function spawn_window(peer: String | null = null) {
  invoke("plugin:owlnest-messaging|spawn_window", { peer }).catch(
    isBodylessHandler
  );
}
let interval_id = setInterval(update_list, 5000);
onUnmounted(() => {
  clearInterval(interval_id);
});
update_list();
</script>
<template>
  <section>
    <section
      class="flex flex-row justify-between items-center border px-4 py-2"
    >
      <p class="hover:cursor-pointer w-[20%]" @click="() => spawn_window()">
        Messaging
      </p>
      <p>Number of reachable peers: {{ connected_peers.length }}</p>
      <div class="hover:cursor-pointer" @click="toggleExpand">
        <span
          class="material-icons float-right"
          style="transform: rotate(90deg)"
          v-if="expand"
        >
          chevron_right
        </span>
        <span v-else class="material-icons float-right"> chevron_right </span>
      </div>
    </section>
    <ul v-if="expand" class="mx-2 border-x border-b">
      <li v-if="connected_peers.length < 1" class="p-2">
        No peer supports this protocol
      </li>
      <li
        v-for="peer in connected_peers"
        class="p-2"
        @click="() => spawn_window(peer)"
      >
        <p class="font-mono">{{ peer }}</p>
      </li>
    </ul>
  </section>
</template>
