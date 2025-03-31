<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { ref, Ref, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import RecvEntry from "./RecvEntry.vue";
import SendEntry from "./SendEntry.vue";
import { isBodylessHandler } from "../../../utils.js";
import { RsSendInfo, RsRecvInfo, PendingSend, PendingRecv } from "./types.ts";

const route = useRoute();
const peer_to_send: Ref<String> = ref(
  route.query.remote ? (route.query.remote as string) : ""
);
const file_path = ref("");
const pending_send = ref(new Map<Number, PendingSend>());
const pending_recv = ref(new Map<Number, PendingRecv>());
const file_drop_listener = ref(() => { });
const incoming_file_rlisten = ref(() => { });

getCurrentWebview().onDragDropEvent((ev) => {
  handle_drop(ev.payload);
}).then((handle) => (file_drop_listener.value = handle))
  .catch(isBodylessHandler);
listen("owlnest-blob-transfer-emit", (ev: any) => {
  if (ev.payload?.IncomingFile) {
    pending_recv.value.set(ev.payload.IncomingFile.local_recv_id, {
      source_peer: ev.payload.IncomingFile.from,
      file_name: ev.payload.IncomingFile.file_name,
      recv_id: ev.payload.IncomingFile.local_recv_id,
      bytes_total: ev.payload.IncomingFile.bytes_total,
    });
    return;
  }
  if (ev.payload?.CancelledRecv) {
    if (!pending_recv.value.delete(ev.payload.CancelledRecv.local_recv_id)) {
      console.error("Incorrect internal state!");
    }
    return;
  }
  if (ev.payload?.CancelledSend) {
    if (!pending_send.value.delete(ev.payload.CancelledSend.local_send_id)) {
      console.error("Incorrect internal state!");
    }
    return;
  }
})
  .then((handle) => (incoming_file_rlisten.value = handle))
  .catch(isBodylessHandler);

invoke("plugin:owlnest-blob-transfer|list_pending_send").then((v: any) => {
  v.forEach((item: RsSendInfo) => {
    pending_send.value.set(item.local_send_id, {
      file_path: item.file_path,
      send_id: item.local_send_id,
      destination: item.remote,
    });
  });
});
invoke<Array<RsRecvInfo>>(
  "plugin:owlnest-blob-transfer|list_pending_recv"
).then((v) => {
  for (let item of v) {
    pending_recv.value.set(item.local_recv_id, {
      file_name: item.file_name,
      recv_id: item.local_recv_id,
      bytes_total: item.bytes_total,
      source_peer: item.remote,
    });
  }
});
function send() {
  if (!(peer_to_send.value && file_path.value)) {
    return;
  }
  invoke<Number>("plugin:owlnest-blob-transfer|send", {
    peer: peer_to_send.value,
    filePath: file_path.value,
  }).then((local_send_id) => {
    pending_send.value.set(local_send_id, {
      file_path: file_path.value,
      send_id: local_send_id,
      destination: peer_to_send.value,
    });
  });
}
onUnmounted(() => {
  file_drop_listener.value();
  incoming_file_rlisten.value();
});
function handle_drop(ev: any) {
  console.log(ev);
  // file_path.value = ev.payload[0].replaceAll("\\", "/");
}
</script>
<template>
  <section class="px-8 py-4">
    <div class="w-full text-center h-12 bg-gray-300">
      <p v-if="!file_path" class="select-none">Drop your file on the window</p>
      <p v-else>{{ file_path }}</p>
    </div>
    <section class="single-input">
      <input v-model="peer_to_send" /><button @click="send">Send</button>
    </section>
  </section>
  <div class="grid grid-cols-2 text-center select-none" style="height: calc(100vh - 8rem)">
    <section>
      <p>Pending send</p>
      <ul class="m-2 p-2 overflow-auto" v-if="pending_send.size > 0">
        <li v-for="item in pending_send.values()" class="mx-2 mt-2 border-gray-300 rounded-md shadow-md">
          <SendEntry :file-path="item.file_path" :send-id="item.send_id"></SendEntry>
        </li>
      </ul>
      <p v-else class="p-2">No item</p>
    </section>
    <section>
      <p>Pending recv</p>
      <ul class="m-2 p-2 overflow-auto" v-if="pending_recv.size > 0">
        <li v-for="item in pending_recv.values()" class="mx-2 mt-2 rounded-md shadow-md">
          <RecvEntry :file-name="item!.file_name" :recv-id="item!.recv_id" :bytes-total="item!.bytes_total"></RecvEntry>
        </li>
      </ul>
      <p v-else class="p-2">No item</p>
    </section>
  </div>
</template>
