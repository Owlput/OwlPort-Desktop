<script setup>
import { invoke } from "@tauri-apps/api";
import { onDeactivated, ref } from "vue";
let expand = ref(false);
let connected_peers = ref({});
invoke("plugin:owlnest-messaging|setup").then(
  (peers) => (connected_peers.value = peers)
);
onDeactivated(() => {
  expand.value = false;
});
function toggleExpand() {
  if (!expand.value)
    invoke("plugin:owlnest-messaging|setup").then(
      (peers) => (connected_peers.value = peers)
    );
  expand.value = !expand.value;
}
</script>
<template>
  <section>
    <section
      class="flex flex-row justify-between items-center border px-4 py-2"
      @click="toggleExpand"
    >
      <p class="hover:cursor-pointer" @click="()=>$router.push('/protocols/messaging')">Messaging</p>
      <p>
        Number of reachable peers: {{ Object.keys(connected_peers).length }}
      </p>
      <span
        class="material-icons"
        style="transform: rotate(90deg)"
        v-if="expand"
      >
        chevron_right
      </span>
      <span v-else class="material-icons"> chevron_right </span>
    </section>
    <ul v-if="expand" class="mx-2 border-x border-b">
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
