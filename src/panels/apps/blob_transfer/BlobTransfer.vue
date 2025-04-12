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
    console.log(local_send_id)
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
  if (ev.type === "drop") file_path.value = ev.paths[0].replaceAll("\\", "/");
}
</script>
<template>
  <section class="mx-8 my-4">
    <div class="w-full text-center h-12 border-dashed rounded-2xl border-2">
      <p v-if="!file_path" class="mt-2 select-none text-xl">Drop your file on the window</p>
      <p v-else class="mt-2 text-xl text-center">{{ file_path }}</p>
      <v-tooltip activator="parent" location="bottom" open-on-hover open-delay="1000">
        Drop your file on the window to read its path
      </v-tooltip>
    </div>
    <form class="single-input mt-2" @submit.prevent="send">
      <v-text-field v-model="peer_to_send" label="Peer ID" />
      <v-btn size="large" height="3.5rem" type="submit">Send</v-btn>
    </form>
  </section>
  <div class="grid grid-cols-2 text-center select-none mx-8 gap-4" style="height: calc(100vh - 14rem)">
    <section>
      <p class="border border-gray-200 shadow-sm rounded-full mx-auto">Pending send</p>
      <ul class="mt-1 h-full overflow-auto border border-gray-200 shadow-sm rounded-sm ">
        <p v-if="pending_send.size < 1" class="p-2">No item</p>
        <li v-for="item in pending_send.values()" class="mx-2 mt-2 border-gray-300 rounded-md shadow-md">
          <SendEntry :file-path="item.file_path" :send-id="item.send_id"></SendEntry>
        </li>
      </ul>
    </section>
    <section>
      <p class="border border-gray-200 shadow-sm rounded-full mx-auto">Pending recv</p>
      <ul class="mt-1 h-full overflow-auto border border-gray-200 shadow-sm rounded-sm ">
        <p v-if="pending_recv.size < 1" class="p-2">No item</p>
        <li v-for="item in pending_recv.values()" class="mx-2 mt-2 rounded-md shadow-md">
          <RecvEntry :file-name="item!.file_name" :recv-id="item!.recv_id" :bytes-total="item!.bytes_total"></RecvEntry>
        </li>
      </ul>
    </section>
  </div>
</template>
