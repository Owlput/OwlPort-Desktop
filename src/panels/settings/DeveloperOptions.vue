<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
</script>

<template>
  <button @click="() => invoke('plugin:popup-tester|emit_test_event')">
    Test popup
  </button>
  <button @click="() => invoke('plugin:owlnest-messaging|init_subapp')">
    init subwindow
  </button>
</template>
