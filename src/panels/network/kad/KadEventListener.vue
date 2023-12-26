<script setup>
import { onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
defineOptions({ name: "ListenEventListener" });

let kad_events = ref([]);
let handle1 = await listen("owlnest-kad-emit", (ev) => {
  kad_events.value.push(ev.payload);
  if (kad_events.value.length > 50){
    kad_events.value.splice(0,1)
  }
  let element = document.getElementById("kad-event-listener") 
  setTimeout(() => {
    element?.scrollTo(0,element.scrollHeight)
  }, 0);
});
onUnmounted(() => {
  handle1();
});
</script>
<template>
  <ul class="shadow-md rounded-md min-h-8 my-4 h-[100%] w-full overflow-auto text-autowrap" id="kad-event-listener">
    <template v-for="event in kad_events">
      <li v-if="event.RoutingUpdated" class="bg-green-300 p-2 mx-4 my-2 rounded-md shadow-md">
        <p>Peer ID: {{ event.RoutingUpdated.peer }}</p>
        <p>Addresses: {{ JSON.stringify(event.RoutingUpdated.addresses) }}</p>
      </li>
    </template>
  </ul>
</template>
