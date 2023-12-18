<script setup>
import { onUnmounted } from "vue";
import Messaging from "./Messaging.vue";
onUnmounted(() => {
  console.log("ProtocolOverview destroyed");
});
</script>

<template>
  <ul
    class="w-full h-full p-4 select-none"
    v-show="$route.path == '/protocols'"
  >
    <KeepAlive>
      <Suspense><Messaging /></Suspense>
    </KeepAlive>
  </ul>
  <RouterView v-slot="{ Component }"
    ><KeepAlive><component :is="Component" /></KeepAlive
  ></RouterView>
</template>
