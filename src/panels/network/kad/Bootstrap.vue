<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import KadEventListener from "./KadEventListener.vue";

</script>
<template>
  <section class="flex py-2 px-12 gap-4 item-evenly-sized">
    <v-btn class="py-2 w-16" @click="() => invoke('plugin:owlnest-kad|insert_default')">
      Insert default nodes
    </v-btn>
    <v-btn @click="() => invoke('plugin:owlnest-kad|bootstrap')">
      Bootstrap
    </v-btn>
  </section>
  <v-divider />
  <section class="" style="height: calc(100vh - 7rem - 3px)">
    <Suspense>
      <KadEventListener />
    </Suspense>
  </section>
</template>
