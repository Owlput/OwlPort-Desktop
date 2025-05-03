<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { isBodylessHandler } from "../../utils";
import AddressDisplay from "../../components/AddressDisplay.vue";

let discovered_nodes: Ref<Map<String, Array<String>>> = ref(new Map());
invoke<any>("plugin:owlnest-mdns|list_discovered")
  .then((v) => {
    Object.keys(v).forEach((k) => discovered_nodes.value.set(k, v[k].map((v: any) => v.split('/p2p/')[0])))
  })
  .catch(isBodylessHandler);
</script>

<template>
  <div>
    <section class="mx-8 mt-4 p-2 shadow-md rounded-md">
      <p class="text-center text-xl select-none">Peers discovered by mDNS</p>
    </section>
    <p v-if="discovered_nodes.size === 0" class="text-center">
      No peer discovered
    </p>
    <ul v-for="peer in discovered_nodes.entries()" class="mx-8 mt-4">
      <li class="shadow-sm border border-gray-100 rounded-sm p-2">
        <section>
          <p class="text-autowrap">Peer ID: {{ peer[0] }}</p>
        </section>
        <ul class="p-2 flex flex-wrap">
          <li class="mx-4 my-1" v-for="addr in peer[1]">
            <AddressDisplay :address="addr.valueOf()" behavior="custom"
              :on-click="() => $router.push(`/main/connections/dial?dial=${addr}`)"
              on-click-desc="Click to dial page." />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
