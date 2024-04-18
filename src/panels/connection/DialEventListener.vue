<script setup>
import { onUnmounted, ref, nextTick } from "vue";
import { listen } from "@tauri-apps/api/event";
defineOptions({ name: "DialEventListener" });

let dial_events = ref([]);
let listener_handle = ref(() => {});

addEventListener("swarm-dial-failed", handleFailedDial);
listen("swarm-emit", (ev) => {
  dial_events.value.push(ev.payload);
  if (dial_events.value.length > 25) {
    dial_events.value.splice(0, 1);
  }
  nextTick(() => {
    let element = document.getElementById("dial-event-listener");
    element?.scrollTo(0, element.scrollHeight);
  });
}).then((handle) => (listener_handle.value = handle));

onUnmounted(() => {
  listener_handle.value();
  removeEventListener("swarm-listen-failed", handleFailedDial);
});

function handleFailedDial(ev) {
  dial_events.value.push({ dialFailed: { reason: ev.detail } });
}
</script>
<template>
  <ul
    class="event-list text-autowrap pb-2 px-4"
    style="height: calc(100% - 2.75rem)"
    id="dial-event-listener"
  >
    <template v-for="event in dial_events">
      <li v-if="event.ConnectionEstablished" class="bg-green-300">
        <p>
          Successfully dialed peer:
          {{ event.ConnectionEstablished.peer_id }}
        </p>
        <p>
          on remote address "{{
            event.ConnectionEstablished.endpoint?.Dialer?.address
          }}"
        </p>
      </li>
      <li v-else-if="event.ConnectionClosed" class="bg-yellow-300">
        <p>Connection to peer {{ event.ConnectionClosed.peer_id }} closed</p>
        <p>
          on remote address "{{
            JSON.stringify(event.ConnectionClosed.endpoint)
          }}"
        </p>
        <p>Caused by {{ event.ConnectionClosed.cause }}</p>
      </li>
      <li v-else-if="event.Dialing" class="bg-blue-300">
        <p>
          Dialing peer
          {{
            event.Dialing.maybe_peer_id ? event.Dialing.maybe_peer_id : "None"
          }}
        </p>
      </li>
      <li v-else-if="event.dialFailed" class="bg-red-300">
        <p>{{ event.dialFailed.reason }}</p>
      </li>
      <li v-else-if="event.OutgoingConnectionError" class="bg-red-300">
        <p>
          Connection to address
          {{ event.OutgoingConnectionError.error?.Transport?.[0][0] }} failed
        </p>
        <p>
          Reason: {{ event.OutgoingConnectionError.error?.Transport?.[0][1] }}
        </p>
      </li>
    </template>
  </ul>
</template>
