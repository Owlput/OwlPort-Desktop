<script setup>
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { ref, onUnmounted } from "vue";
const peer_to_send = ref("");
const file_path = ref("");
const pending_send = ref([]);
const pending_recv = ref([]);
const listener = ref(() => {});
listen("tauri://file-drop", (ev) => {
  handle_drop(ev);
}).then((handle) => (listener.value = handle));
function update_pending() {
  setTimeout(() => {
    invoke("plugin:owlnest-blob-transfer|list_pending_send").then(
      (v) => (pending_send.value = v)
    );
    invoke("plugin:owlnest-blob-transfer|list_pending_recv").then(
      (v) => (pending_recv.value = v)
    );
  },50);
}
update_pending();
function send() {
  if (!(peer_to_send.value && file_path.value)) {
    return;
  }
  invoke("plugin:owlnest-blob-transfer|send", {
    peer: peer_to_send.value,
    filePath: file_path.value,
  });
  update_pending()
}

onUnmounted(() => {
  listener.value();
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
          v-for="item in pending_send"
          class="p-2 border border-gray-300 rounded-sm"
        >
          <p>{{ item.file_name }}</p>
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
        <li v-for="item in pending_recv">
          <p>{{ item.file_name }}</p>
          <button
            @click="
              () =>
                invoke('plugin:owlnest-blob-transfer|recv', {
                  recvId: item.local_recv_id,
                  fileName: item.file_name,
                }).then(update_pending)
            "
          >
            Accept
          </button>
          <button
            @click="
              () =>
                invoke('plugin:owlnest-blob-transfer|cancel_recv', {
                  recvId: item.local_recv_id,
                }).then(update_pending)
            "
          >
            Cancel
          </button>
        </li>
      </ul>
    </section>
  </div>
</template>
