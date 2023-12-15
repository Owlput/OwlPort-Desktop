<script setup>
import { listen } from "@tauri-apps/api/event";
import { ref,onUnmounted } from "vue";


onUnmounted(()=>{
    console.log("ConnectedPeers destroyed")
})
</script>

<template>
  <p>This is connected peers.</p>
</template>
