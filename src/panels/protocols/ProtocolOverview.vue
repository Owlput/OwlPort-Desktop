<script setup>
import { onUnmounted } from "vue";
import MessagingStatus from "./MessagingStatus.vue";
import BlobTransferStatus from "./BlobTransferStatus.vue";
onUnmounted(() => {
  console.log("ProtocolOverview destroyed");
});
</script>

<template>
  <ul
    class="w-full h-full p-4 select-none"
    v-show="$route.path == '/protocols'"
  >
    <li class="p-2">
      <KeepAlive>
        <Suspense><MessagingStatus /></Suspense>
      </KeepAlive>
    </li>
    <li class="p-2"><BlobTransferStatus /></li>
  </ul>
</template>
