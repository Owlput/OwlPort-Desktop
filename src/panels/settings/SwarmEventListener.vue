<script setup lang="ts">
import { onUnmounted, ref, Ref, nextTick } from "vue";
import { listen } from "@tauri-apps/api/event";
import { is_ip_private, isBodylessHandler } from "../../utils";
import { SwarmEmit } from "../connection/types";

defineOptions({ name: "DialEventListener" });

let dial_events: Ref<Array<SwarmEmit>> = ref([]);
let listener_handle = ref(() => { });

listen<SwarmEmit>("swarm-emit", (ev) => {
  dial_events.value.push(ev.payload);
  if (dial_events.value.length > 25) {
    dial_events.value.splice(0, 1);
  }
  nextTick(() => {
    let element = document.getElementById("dial-event-listener");
    element?.scrollTo(0, element.scrollHeight);
  });
})
  .then((handle) => (listener_handle.value = handle))
  .catch(isBodylessHandler);

onUnmounted(() => {
  listener_handle.value();
});

</script>
<template>
  <v-main>
    <ul class="event-list text-autowrap pb-2 px-4 event-list" style="height: calc(100% - 2.75rem)"
      id="dial-event-listener">
      <template v-for="event in dial_events">
        <li v-if="event.ConnectionEstablished" class="bg-green-300">
          <p>
            Successfully dialed peer:
            {{ event.ConnectionEstablished.peer_id }}
          </p>
          <p>
            on remote address {{
              event.ConnectionEstablished.endpoint?.Dialer?.address
            }}
          </p>
        </li>
        <li v-else-if="event.ConnectionClosed" class="bg-yellow-100">
          <p>Connection to peer {{ event.ConnectionClosed.peer_id }} closed</p>
          <p>
            on remote address {{
              event.ConnectionClosed.endpoint.Dialer?.address
            }}
          </p>
          <p v-if="event.ConnectionClosed.cause !== 'None'">Caused by {{ event.ConnectionClosed.cause }}</p>
        </li>
        <li v-else-if="event.Dialing" class="bg-blue-300">
          <p>
            Dialing peer
            {{
              event.Dialing.maybe_peer_id ? event.Dialing.maybe_peer_id : "None"
            }}
          </p>
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
        <li v-else-if="event.NewListenAddr" class="flex gap-2">
          <p>New listening address: </p>
          <v-chip v-if="is_ip_private(event.NewListenAddr.address)" density="compact" color="#66bb6a">
            {{ event.NewListenAddr.address }}
            <v-tooltip activator="parent" location="bottom"> Not reachable on Internet </v-tooltip>
          </v-chip>
          <v-chip v-else density="compact" color="#42a5f5">
            {{ event.NewListenAddr.address }}
            <v-tooltip activator="parent" location="bottom"> Reachable on Internet </v-tooltip>
          </v-chip>
        </li>
        <li v-else>
          {{ event }}
        </li>
      </template>
    </ul>
  </v-main>
</template>

<style scoped>
.event-list>li {
  --tw-shadow: 0 3px 4px -1px rgb(0 0 0 / 0.1),
    0 2px 4px -2px rgb(0 0 0 / 0.1);
  --tw-shadow-colored: 0 4px 6px -1px var(--tw-shadow-color),
    0 2px 4px -2px var(--tw-shadow-color);
  box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000),
    var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow);
  border: solid 1px;
  border-color: #dadada;
}
</style>
