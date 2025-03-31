<script setup lang="ts">
import { ref, PropType } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { PeerStub } from "./types";
import AddressDisplay from "../../../components/AddressDisplay.vue";

const props = defineProps(
  {
    peer: {
      type: Object as PropType<PeerStub>,
      required: true
    }
  }
);

const show_addresses = ref(false);
</script>
<template>
  <div class="flex justify-between flex-nowrap border px-2 cursor-pointer"
    @click.prevent.self="() => (show_addresses = !show_addresses)">
    <p class="select-none cursor-default" @dblclick="writeText(props.peer.peer_id.valueOf())">
      {{ props.peer.peer_id }}
    </p>
  </div>
  <section v-if="show_addresses" class="mx-2 border-x text-autowrap border-b">
    <div class="h-6">
      <p class="px-2 float-left">
        Reachable addresses({{ props.peer.addresses.length }}):
      </p>
    </div>
    <ul class="px-4 overflow-auto">
      <li v-if="props.peer.addresses.length === 0">
        No listenable address(Addresses not public)
      </li>
      <li v-for="addr in props.peer.addresses" class="my-0 w-full">
        <AddressDisplay :address="addr.valueOf()" />
      </li>
    </ul>
  </section>
</template>
