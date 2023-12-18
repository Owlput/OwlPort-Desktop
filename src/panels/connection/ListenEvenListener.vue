<script setup>
import { onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
defineOptions({ name: "ListenEventListener" });

let listen_events = ref([]);
let handle1 = await listen("swarm-event", (ev) => {
  console.log(ev);
  listen_events.value.push(ev.payload);
});
addEventListener("swarm-listen-failed", handleFailedListen);
onUnmounted(() => {
  handle1();
  removeEventListener("swarm-listen-failed", handleFailedListen);
});
function handleFailedListen(ev) {
  listen_events.value.push({ listenFailed: { reason: ev.detail } });
}
</script>
<template>
  <ul class="shadow-md rounded-md min-h-8 my-4">
    <template v-for="event in listen_events">
      <li v-if="event.NewListenAddr" class="bg-green-300 p-2">
        <p>
          Now listening on
          {{ event.NewListenAddr.address }}
        </p>
      </li>
      <li v-if="event.ExpiredListenAddr" class="bg-yellow-300 p-2">
        <p>
          Listener on address
          {{ event.ConnectionClosed.peer_id }} expired
        </p>
      </li>
      <li v-if="event.listenFailed" class="bg-red-300 p-2">
        <p>{{ event.listenFailed.reason }}</p>
      </li>
    </template>
  </ul>
</template>
