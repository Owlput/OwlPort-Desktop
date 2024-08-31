<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, Ref } from "vue";
import { isBodylessHandler } from "../../../utils";
import {useRouter} from "vue-router"

const router = useRouter();

let connected_peers: Ref<Array<String>> = ref([]);
invoke<Array<String>>("plugin:owlnest-messaging|list_connected")
  .then((peers) => (connected_peers.value = peers))
  .catch(isBodylessHandler);

</script>
<template>
  <section class="text-center">
    <p class="mt-[20%] text-3xl">Feel free to chat</p>
    <p>Below are some peers you may start a chat with</p>
    <ul class="mx-2 border">
      <li v-if="Object.keys(connected_peers).length < 1" class="p-2">
        No peer supports this protocol
      </li>
      <li
        v-for="peer in connected_peers"
        class="p-2"
        @click="() => router.push(`/app/messaging/${peer}`)"
      >
        {{ peer }}
      </li>
    </ul>
  </section>
</template>
