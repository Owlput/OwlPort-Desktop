<script setup lang="ts">
import { ref } from "vue";
import { writeText } from "@tauri-apps/api/clipboard";

const props = defineProps<{
  peerId: String;
  addresses: Array;
}>();

const show_addresses = ref(false);
</script>
<template>
  <div
    class="flex justify-between flex-nowrap border px-2 cursor-pointer"
    @click.prevent.self="() => (show_addresses = !show_addresses)"
  >
    <p class="select-none cursor-default" @dblclick="writeText(props.peerId)">
      {{ props.peerId }}
    </p>
  </div>
  <section v-if="show_addresses" class="mx-2 border-x text-autowrap border-b">
    <div class="h-6">
      <p class="px-2 float-left">
        Reachable addresses({{ props.addresses.length }}):
      </p>
    </div>
    <ul class="px-4 overflow-auto">
      <li v-if="props.addresses.length === 0">
        No listenable address(Addresses not public)
      </li>
      <li v-for="addr in props.addresses" class="my-0 w-full">
        <p @dblclick="writeText(addr)" class="text-autowrap">
          {{ addr }}
        </p>
      </li>
    </ul>
  </section>
</template>
