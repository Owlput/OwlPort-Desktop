<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import KadEventListener from "./KadEventListener.vue";

</script>
<template>
  <section class="flex py-4 px-12 border-b item-evenly-sized">
    <button
      class="py-2 w-16"
      @click="() => invoke('plugin:owlnest-kad|insert_default')"
    >
      Insert default nodes
    </button>
    <button @click="() => invoke('plugin:owlnest-kad|bootstrap')">
      Bootstrap
    </button>
  </section>
  <section class="" style="height: calc(100vh - 8rem - 3px)">
    <Suspense>
      <KadEventListener />
    </Suspense>
  </section>
</template>
