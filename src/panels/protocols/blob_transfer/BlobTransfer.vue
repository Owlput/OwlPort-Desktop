<script setup>
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { ref, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import RecvEntry from "./RecvEntry.vue";
import SendEntry from "./SendEntry.vue";
const route = useRoute();
const peer_to_send = ref(route.query.remote ? route.query.remote : "");
const file_path = ref("");
const pending_send = ref(new Map());
const pending_recv = ref(new Map());
const file_drop_listener = ref(() => {});
const incoming_file_rlisten = ref(() => {});
listen("tauri://file-drop", (ev) => {
  handle_drop(ev);
}).then((handle) => (file_drop_listener.value = handle));
listen("owlnest-blob-transfer-emit", (ev) => {
  if (ev.payload?.IncomingFile) {
    pending_recv.value.set(ev.payload.IncomingFile.local_recv_id, {
      from: ev.payload.IncomingFile.from,
      fileName: ev.payload.IncomingFile.file_name,
      recvId: ev.payload.IncomingFile.local_recv_id,
      bytesTotal: ev.payload.IncomingFile.bytes_total,
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
}).then((handle) => (incoming_file_rlisten.value = handle));
invoke("plugin:owlnest-blob-transfer|list_pending_send").then((v) => {
  v.forEach((item) => {
    pending_send.value.set(item.local_send_id, {
      filePath: item.file_path,
      sendId: item.local_send_id,
    });
  });
});
invoke("plugin:owlnest-blob-transfer|list_pending_recv").then((v) => {
  for (let item of v) {
    pending_recv.value.set(item.local_recv_id, {
      fileName: item.file_name,
      recvId: item.local_recv_id,
      bytesTotal: item.bytes_total,
    });
  }
});
function send() {
  if (!(peer_to_send.value && file_path.value)) {
    return;
  }
  invoke("plugin:owlnest-blob-transfer|send", {
    peer: peer_to_send.value,
    filePath: file_path.value,
  }).then((local_send_id) => {
    pending_send.value.set(local_send_id, {
      filePath: file_path,
      sendId: local_send_id,
    });
  });
}
onUnmounted(() => {
  file_drop_listener.value();
  incoming_file_rlisten.value();
});
function handle_drop(ev) {
  file_path.value = ev.payload[0].replaceAll("\\", "/");
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
  <div
    class="grid grid-cols-2 text-center select-none"
    style="height: calc(100vh - 8rem)"
  >
    <section>
      <p>Pending send</p>
      <ul class="m-2 p-2 overflow-auto" v-if="pending_send.size > 0">
        <li
          v-for="item in pending_send.values()"
          class="mx-2 mt-2 border-gray-300 rounded-md shadow-md"
        >
          <SendEntry
            :file-path="item.filePath"
            :send-id="item.sendId"
          ></SendEntry>
        </li>
      </ul>
      <p v-else class="p-2">No item</p>
    </section>
    <section>
      <p>Pending recv</p>
      <ul class="m-2 p-2 overflow-auto" v-if="pending_recv.size > 0">
        <li v-for="item in pending_recv.values()" class="mx-2 mt-2 rounded-md shadow-md">
          <RecvEntry
            :file-name="item.fileName"
            :recv-id="item.recvId"
            :bytes-total="item.bytesTotal"
          ></RecvEntry>
        </li>
      </ul>
      <p v-else class="p-2">No item</p>
    </section>
  </div>
</template>
