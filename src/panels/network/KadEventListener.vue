<script setup>
import { onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
defineOptions({ name: "ListenEventListener" });

let kad_events = ref([]);
let handle1 = await listen("owlnest-kad-emit", (ev) => {
  kad_events.value.push(ev.payload);
});
onUnmounted(() => {
  handle1();
});
</script>
<template>
  <ul class="shadow-md rounded-md min-h-8 my-4 h-[20rem] w-full overflow-auto text-autowrap">
    <template v-for="event in kad_events">
      <li v-if="event.RoutingUpdated" class="bg-green-300 p-2 mx-4 my-2 rounded-md shadow-md">
        {{ event.RoutingUpdated }}
      </li>
    </template>
  </ul>
</template>
