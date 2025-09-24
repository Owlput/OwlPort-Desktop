<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Ref, ref } from "vue";

const status: Ref<string | null> = ref(null);

function generateDefaultConfig() {
  invoke("plugin:owlport-config-writer|write_default_config", {
    path: "./owlnest_config.toml.example",
  })
    .then((_v) => {
      status.value =
        "Example configuration written to disk. Please rename the file and relaunch the application.";
    })
    .catch((e) => (status.value = e));
}
</script>

<template>
  <h1 class="text-center text-2xl my-4">No Valid Cofiguration File Found</h1>
  <p class="text-center">
    Place your owlnest_config.toml file <br />
    under the same directory as the OwlPort binary, <br />
    or generate a default one.
  </p>
  <form
    @submit.prevent="generateDefaultConfig"
    class="w-full flex justify-center py-2"
  >
    <v-btn type="submit">Generate Config File</v-btn>
  </form>
  <p v-if="status !== null" class="w-full px-4 pt-4 text-center">{{ status }}</p>
</template>
