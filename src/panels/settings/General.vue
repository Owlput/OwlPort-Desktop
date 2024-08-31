<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const showNestConnectionSettings = ref(false);
const generalSettings = ref({});

// function applyGeneralSettings() {
//   invoke("plugin:configuration|write_config", {
//     config: {AppConfig:generalSettings.value},
//     path: {AppConfig:null},
//   })
//     .then(() => console.log("config sccessfully applied"))
//     .catch((e) => console.log("failed to apply config:", e));
//   document.activeElement.blur();
// }
// onMounted(()=>{
//   invoke("plugin:configuration|read_config",{path:{AppConfig:null}}).then(
//     (v)=>generalSettings.value = JSON.parse(v).AppConfig
//   ).catch((e)=>console.error(e))
// })
</script>
<template>
  <div class="px-16 py-4">
    Test
  </div>
</template>
