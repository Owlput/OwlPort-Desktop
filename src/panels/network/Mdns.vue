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
    <section class="mx-[3rem] my-4 p-2 shadow-md rounded-md">
      <p class="text-center">Peers discovered by mDNS</p>
      <section class="flex gap-2">
        <p class="bg-blue-300 p-1 rounded-sm">Public addresses</p>
        <p class="bg-green-300 p-1 rounded-sm">Local addresses</p>
      </section>
    </section>

    <p v-if="discovered_nodes.size === 0" class="text-center">
      No peer discovered
    </p>
    <ul v-for="peer in discovered_nodes.entries()" class="m-8 border">
      <li>
        <section>
          <p class="text-autowrap">Peer ID: {{ peer[0] }}</p>
        </section>
        <ul class="p-4 flex flex-wrap">
          <li class="mx-4 my-1 cursor-pointer" v-for="addr in peer[1]"
            @click="() => $router.push(`/main/connections/dial?dial=${addr}`)">
            <AddressDisplay :address="addr.valueOf()" />
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
