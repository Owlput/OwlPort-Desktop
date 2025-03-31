<script setup lang="ts">
import { ref } from "vue";
const pathToCertificate = ref("certs/");
const kacUtil = ref(false);
function toggleKacUtil() {
  kacUtil.value = true;
  console.log("dialog toggled");
}
</script>
<template></template>
