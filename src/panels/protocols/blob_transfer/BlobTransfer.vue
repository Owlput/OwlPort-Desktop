<script setup>
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { ref, onUnmounted } from "vue";
import { useRoute } from "vue-router";
const route = useRoute();
const peer_to_send = ref(route.query?.remote ? route.query.remote : "");
const file_path = ref("");
const pending_send = ref(new Map());
const pending_recv = ref(new Map());
const file_drop_listener = ref(() => {});
const incoming_file_listener = ref(() => {});
listen("tauri://file-drop", (ev) => {
  handle_drop(ev);
}).then((handle) => (file_drop_listener.value = handle));
listen("owlnest-blob-transfer-emit", (ev) => {
  if (ev.payload?.IncomingFile) {
    pending_recv.value.set(ev.payload.IncomingFile.local_recv_id, {
      from: ev.payload.IncomingFile.from,
      file_name: ev.payload.IncomingFile.file_name,
      local_recv_id: ev.payload.IncomingFile.local_recv_id,
      bytes_total: ev.payload.IncomingFile.bytes_total,
      bytes_sent: null,
    });
    return;
  }
  if (ev.payload?.CancelledRecv) {
    console.log(ev)
    if (!pending_recv.value.delete(ev.payload.CancelledRecv)){
      console.error("Incorrect internal state!")
    };
    return;
  }
  if (ev.payload?.CancelledSend) {
    if (!pending_send.value.delete(ev.payload.CancelledSend)){
      console.error("Incorrect internal state!")
    }
    return
  }
  if (ev.payload?.SendProgressed) {
    let entry = pending_send.value.get(ev.payload.SendProgressed.local_send_id);
    if (!entry) {
      console.error("Incorrect internal state!");
      return;
    }
    entry.bytes_sent = ev.payload.SendProgressed.bytes_sent;
    return;
  }
  if (ev.payload?.RecvProgressed) {
    let entry = pending_recv.get(ev.payload.RecvProgressed.local_recv_id);
    if (!entry) {
      console.error("Incorrect internal state!");
      return;
    }
    entry.bytes_received = ev.payload.RecvProgressed.bytes_received;
    return;
  }
}).then((handle) => (incoming_file_listener.value = handle));
function send() {
  if (!(peer_to_send.value && file_path.value)) {
    return;
  }
  invoke("plugin:owlnest-blob-transfer|send", {
    peer: peer_to_send.value,
    filePath: file_path.value,
  }).then((local_send_id) => {
    let path_parts = file_path.value.split("/");
    pending_send.value.set(local_send_id, {
      file_name: path_parts[path_parts.length - 1],
      local_send_id,
      bytes_total: null,
      bytes_sent: 0,
    });
  });
}

onUnmounted(() => {
  file_drop_listener.value();
  incoming_file_listener.value();
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
  <div class="grid grid-cols-2 text-center select-none">
    <section>
      <p>Pending send</p>
      <ul class="p-2">
        <li
          v-for="item in pending_send.values()"
          class="p-2 border border-gray-300 rounded-sm"
        >
          <p>{{ item.file_name }}</p>
          <p v-if="item.bytes_total">
            {{ item.bytes_sent / item.bytes_total }}%
          </p>
          <button
            @click="
              () => {
                invoke('plugin:owlnest-blob-transfer|cancel_send', {
                  sendId: item.local_send_id,
                }).then(update_pending);
              }
            "
          >
            Cancel
          </button>
        </li>
      </ul>
    </section>
    <section>
      <p>Pending recv</p>
      <ul>
        <li v-for="item in pending_recv.values()">
          <p>{{ item.file_name }}</p>
          <p>{{ item.bytes_total / 1024 / 1024 }}MB</p>
          <button
            @click="
              () =>
                invoke('plugin:owlnest-blob-transfer|recv', {
                  recvId: item.local_recv_id,
                  fileName: item.file_name,
                })
            "
          >
            Accept
          </button>
          <button
            @click="
              () =>
                invoke('plugin:owlnest-blob-transfer|cancel_recv', {
                  recvId: item.local_recv_id,
                }).catch((e)=> console.error(e))
            "
          >
            Cancel
          </button>
        </li>
      </ul>
    </section>
  </div>
</template>
