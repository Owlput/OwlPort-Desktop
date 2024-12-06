<script setup lang="ts">
import { ref, onUnmounted, Ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { isBodylessHandler } from "../../../utils.js";

const props = defineProps<{
  fileName: String;
  recvId: Number;
  bytesTotal: Number;
}>();
const accepted = ref(false);
const progress: Ref<Number | null> = ref(null);
const recv_progress_rlisten: Ref<Function | null> = ref(null);
listen("owlnest-blob-transfer-emit", (ev: any) => {
  if (ev.payload?.RecvProgressed) {
    console.log(ev.payload.RecvProgressed);
    progress.value =
      (ev.payload.RecvProgressed.bytes_received / props.bytesTotal.valueOf()) * 100;
  }
}).then((handle) => {
  recv_progress_rlisten.value = handle;
}).catch(isBodylessHandler);
onUnmounted(() => {
  if (recv_progress_rlisten.value) {
    recv_progress_rlisten.value!();
  }
});
</script>
<template>
  <p>{{ props.fileName }}</p>
  <p>{{ (props.bytesTotal.valueOf() / 1024 / 1024).toFixed(2) }}MB</p>
  <p v-if="progress">{{ progress }}%</p>
  <button :disabled="accepted" class="p-1 w-[50%] shadow-none" @click="() =>
      invoke('plugin:owlnest-blob-transfer|recv', {
        recvId: props.recvId,
        fileName: props.fileName,
      })
    ">
    Accept
  </button>
  <button class="p-1 w-[50%] shadow-none" @click="() =>
      invoke('plugin:owlnest-blob-transfer|cancel_recv', {
        recvId: props.recvId,
      }).catch((e) => console.error(e))
    ">
    Cancel
  </button>
</template>
