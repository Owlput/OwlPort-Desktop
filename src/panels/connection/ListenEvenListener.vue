<script setup>
import { onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
defineOptions({ name: "ListenEventListener" });

let listen_events = ref([]);
let handle = await listen("swarm-event", (ev) => {
  console.log(ev);
  listen_events.value.push(ev);
});
onUnmounted(() => {
  handle();
});
</script>
<template>
  <ul class="shadow-md rounded-md min-h-8 my-4">
    <template v-for="event in listen_events">
      <li v-if="event.payload.NewListenAddr" class="bg-green-300 p-2">
        <p>
          Now listening on 
          {{ event.payload.NewListenAddr.address }}
        </p>
      </li>
      <li v-if="event.payload.ExpiredListenAddr" class="bg-yellow-300 p-2">
        <p>
          Listener on address
          {{ event.payload.ConnectionClosed.peer_id }} expired
        </p>
      </li>
    </template>
  </ul>
</template>
