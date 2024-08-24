<script setup>
import { onUnmounted } from "vue";
import MessagingStatus from "./MessagingStatus.vue";
import BlobTransferStatus from "./BlobTransferStatus.vue";
import GenericAppStatus from "./GenericAppStatus.vue"
onUnmounted(() => {
  console.log("ProtocolOverview destroyed");
});
</script>

<template>
  <ul class="w-full h-full p-4 select-none">
    <li class="p-2">
      <MessagingStatus />
    </li>
    <li class="p-2">
      <BlobTransferStatus />
    </li>
    <li class="p-2">
      <GenericAppStatus :app-name="'Gossip-Sub Messaging'" :window-entrypoint="'plugin:libp2p-gossipsub|spawn_window'"/>
    </li>
  </ul>
</template>
