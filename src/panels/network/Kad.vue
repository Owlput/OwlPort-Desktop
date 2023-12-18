<script setup>
import { invoke } from '@tauri-apps/api';
import KadEventListener from './KadEventListener.vue';
</script>
<template>
  <section>
    <button @click="()=>invoke('plugin:owlnest-kad|insert_default')">Insert default nodes</button>
    <button @click="()=>invoke('plugin:owlnest-kad|bootstrap')">Bootstrap</button>
  </section>
  <section>
    <Suspense>
        <KadEventListener />
    </Suspense>
  </section>
</template>
