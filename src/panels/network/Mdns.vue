<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { isBodylessHandler } from "../../utils";

let discovered_nodes = ref({});
invoke("plugin:owlnest-mdns|list_discovered")
  .then((v) => {
    discovered_nodes.value = v;
  })
  .catch(isBodylessHandler);
</script>

<template>
  <div>
    <section class="mx-[3rem] my-4 p-2 shadow-md rounded-md">
      <p class="text-center">Peers discovered by mDNS</p>
    </section>

    <p v-if="Object.keys(discovered_nodes).length < 1" class="text-center">
      No peer discovered
    </p>
    <ul v-for="peer in Object.keys(discovered_nodes)" class="p-2 border">
      <li>
        <section>
          <p class="text-autowrap">Peer ID: {{ peer }}</p>
        </section>
        <ul class="p-4 bg-slate-200">
          <li
            v-for="addr in discovered_nodes[peer]"
            class="w-full text-autowrap"
            @click="() => $router.push(`/main/connections/dial?dial=${addr}`)"
          >
            <p class="text-autowrap">{{ addr }}</p>
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
