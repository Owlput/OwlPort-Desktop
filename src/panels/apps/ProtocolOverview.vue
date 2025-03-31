<script setup lang="ts">
import MessagingStatus from "./MessagingStatus.vue";
import BlobTransferStatus from "./BlobTransferStatus.vue";
import GenericAppStatus from "./GenericAppStatus.vue";
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
      <GenericAppStatus
        :app-name="'Gossip-Sub Messaging'"
        :window-entrypoint="'plugin:owlnest-gossipsub|spawn_window'"
        :alternative-path="'/app/gossipsub'"
      />
    </li>
  </ul>
</template>
