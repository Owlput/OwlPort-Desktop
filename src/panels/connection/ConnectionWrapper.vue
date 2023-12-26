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
  <ul class="tab-l-container">
    <li
      class="p-2"
      @click="
        () => {
          $router.push('/connections/dial');
        }
      "
    >
      Dial
      </li>
    <li
      class="p-2"
      @click="
        () => {
          $router.push('/connections/listen');
        }
      "
    >
      Listen</li
    ><li
      class="p-2"
      @click="
        () => {
          $router.push('/connections/connected');
        }
      "
    >
      Connected
    </li>
  </ul>
  <div class="w-[100%]" style="height: calc(100vh - 3rem);">
    <RouterView v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </div>
</template>
