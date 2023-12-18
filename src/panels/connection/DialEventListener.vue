<script setup>
import { onUnmounted ,ref} from 'vue';
import {listen} from "@tauri-apps/api/event"
defineOptions({name:"DialEventListener"});

let dial_events = ref([]);
let handle = await listen("swarm-event", (ev) => {
  dial_events.value.push(ev);
});
onUnmounted(()=>{
  handle()
})
</script>
<template>
    <ul class="shadow-md rounded-md min-h-8 my-4">
        <template v-for="event in dial_events">
          <li
            v-if="event.payload.ConnectionEstablished"
            class="bg-green-300 p-2"
          >
            <p>
              Successfully dialed peer:
              {{ event.payload.ConnectionEstablished.peer_id }}
            </p>
            <p>
              on remote address "{{
                event.payload.ConnectionEstablished.endpoint.Dialer.address
              }}"
            </p>
          </li>
          <li v-if="event.payload.ConnectionClosed" class="bg-yellow-300 p-2">
            <p>Connection to peer {{ event.payload.ConnectionClosed.peer_id }} closed</p>
            <p>on remote address "{{ JSON.stringify(event.payload.ConnectionClosed.endpoint) }}"</p>
            <p>Caused by {{ event.payload.ConnectionClosed.cause }}</p>
        </li>
            <li v-if="event.payload.Dialing" class="bg-blue-300 p-2">
            <p>Dialing peer {{ event.payload.Dialing.maybe_peer_id?event.payload.Dialing.maybe_peer_id:"None" }}</p>
        </li>
        </template>
      </ul>
</template>