<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();
</script>

<template>
  <RouterView v-slot="{ Component }">
    <KeepAlive exclude="Overview">
      <component :is="Component" />
    </KeepAlive>
  </RouterView>
</template>
