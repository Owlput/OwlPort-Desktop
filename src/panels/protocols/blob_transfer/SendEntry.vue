<script setup>
import { ref, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

const props = defineProps({
  filePath: String,
  sendId: Number,
});
const progress = ref(null);
const progress_rlisten = ref(null);
listen("owlnest-blob-emit", (ev) => {
  if (ev.payload?.SendProgressed) {
    progress.value =
      ev.payload.SendProgressed.bytes_sent /
      ev.payload.SendProgressed.bytes_total;
  }
}).then((handle) => {
  progress_rlisten.value = handle;
});
onUnmounted(() => {
  progress_rlisten.value();
});
</script>

<template>
  <section class="w-full">
    <p class="overflow-auto">{{ props.filePath }}</p>
    <p></p>
    <p v-if="progress">Progress: {{ progress.toFixed(2) }}%</p>
    <button
      @click="
        () => {
          invoke('plugin:owlnest-blob-transfer|cancel_send', {
            sendId: props.sendId,
          });
        }
      "
      class="shadow-none w-full border-t mt-2 p-1"
    >
      Cancel
    </button>
  </section>
</template>
