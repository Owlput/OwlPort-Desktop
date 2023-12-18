<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
let discovered_nodes = ref({});
invoke("plugin:owlnest-mdns|list_discovered").then((v) => {
  console.log(v);
  discovered_nodes.value = v;
});
console.log(discovered_nodes);
</script>

<template>
  <div class="p-2">
    <p>Peers discovered by mDNS:</p>
    <ul v-for="peer in Object.keys(discovered_nodes)" class="m-2 border">
      <li>
        <section ><p class="text-autowrap">Peer ID: {{ peer }}</p></section>
        <ul class="m-4 bg-slate-200">
          <li
            v-for="addr in discovered_nodes[peer]"
            class="w-full text-autowrap"
            @click="()=>$router.push(`/connections/dial?dial=${addr}`)"
          >
           <p class="text-autowrap">{{ addr }}</p> 
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
