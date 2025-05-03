<script setup lang="ts">
import { emit, listen } from '@tauri-apps/api/event';
import { MessagingEmit } from './messaging/types';
import { Popup } from '../../components/Types';
import PopUpHandler from '../../components/PopUpHandler.vue';
import { onUnmounted } from 'vue';
import { BlobTransferEmit } from './blob_transfer/types';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { useRoute } from 'vue-router';
import { isBodyless } from '../../utils';

let appWindowLabel;
if (!isBodyless()) { console.log("backend connected"); appWindowLabel = getCurrentWebviewWindow().label; } else {
  let route = useRoute();
  appWindowLabel = route.fullPath.split("/")[1]
}

let event_handles: Array<Function> = []
if (appWindowLabel === "messaging") {
  listen<MessagingEmit>("owlnest-messaging-emit", (ev) => {
    if (ev.payload.IncomingMessage) {
      emit("newPopup", new Popup(Date.now(), "DefaultPopup", { message: "Incoming message. Open Apps > Messaging to view the message." }))
    }
  }).then((handle) => event_handles.push(handle))
}
if (appWindowLabel === "blob-transfer") {
  listen<BlobTransferEmit>("owlnest-blob-transfer-emit", (ev) => {
    if (ev.payload.IncomingFile) {
      emit("newPopup", new Popup(Date.now(), "DefaultPopup", { message: "Incoming file. Open Apps > Messaging to view the message." }))
    }
  }).then((handle) => event_handles.push(handle))
}
onUnmounted(() => {
  event_handles.forEach((unlisten) => unlisten())
})
</script>

<template>
  <RouterView v-slot="{ Component }">
    <component :is="Component" />
  </RouterView>
  <PopUpHandler></PopUpHandler>
</template>
