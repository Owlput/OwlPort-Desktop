<script setup>
import { onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
defineOptions({ name: "DialEventListener" });

let dial_events = ref([]);
let handle = await listen("swarm-event", (ev) => {
  dial_events.value.push(ev.payload);
});
addEventListener("swarm-dial-failed", handleFailedDial);
onUnmounted(() => {
  handle();
  removeEventListener("swarm-listen-failed", handleFailedDial);
});
function handleFailedDial(ev) {
  dial_events.value.push({ dialFailed: { reason: ev.detail } });
}
</script>
<template>
  <ul class="shadow-md rounded-md min-h-8 my-4">
    <template v-for="event in dial_events">
      <li v-if="event.ConnectionEstablished" class="bg-green-300 p-2">
        <p>
          Successfully dialed peer:
          {{ event.ConnectionEstablished.peer_id }}
        </p>
        <p>
          on remote address "{{
            event.ConnectionEstablished.endpoint.Dialer.address
          }}"
        </p>
      </li>
      <li v-if="event.ConnectionClosed" class="bg-yellow-300 p-2">
        <p>Connection to peer {{ event.ConnectionClosed.peer_id }} closed</p>
        <p>
          on remote address "{{
            JSON.stringify(event.ConnectionClosed.endpoint)
          }}"
        </p>
        <p>Caused by {{ event.ConnectionClosed.cause }}</p>
      </li>
      <li v-if="event.Dialing" class="bg-blue-300 p-2">
        <p>
          Dialing peer
          {{
            event.Dialing.maybe_peer_id ? event.Dialing.maybe_peer_id : "None"
          }}
        </p>
      </li>
      <li v-if="event.dialFailed" class="bg-red-300 p-2">
        <p>{{ event.dialFailed.reason }}</p>
      </li>
    </template>
  </ul>
</template>
