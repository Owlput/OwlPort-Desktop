<script setup>
import { onUnmounted } from "vue";
defineOptions({
  name: "ConnectionWrapper",
});
console.log("Wrapper created");
onUnmounted(() => {
  console.log("Wrapper destroyed");
});
</script>

<template>
  <div class="flex flex-row justify-evenly items-center">
    <button
      @click="
        () => {
          $router.push('/connections/dial');
        }
      "
    >
      Dial</button
    ><button
      @click="
        () => {
          $router.push('/connections/connected');
        }
      "
    >
      Connected
    </button>
  </div>
  <div class="w-[100%]">
    <RouterView v-slot="{ Component }">
      <KeepAlive include="ConnectedPeers,Dial">
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </div>
</template>
