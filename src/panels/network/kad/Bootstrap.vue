<script setup>
import { invoke } from "@tauri-apps/api";
import KadEventListener from "./KadEventListener.vue";
</script>
<template>
  <section class="flex item-evenly-sized p-4">
    <button @click="() => invoke('plugin:owlnest-kad|insert_default')">
      Insert default nodes
    </button>
    <button @click="() => invoke('plugin:owlnest-kad|bootstrap')">
      Bootstrap
    </button>
  </section>
  <section class="" style="height: calc(100vh - 9rem - 2px)">
    <Suspense>
      <KadEventListener />
    </Suspense>
  </section>
</template>
