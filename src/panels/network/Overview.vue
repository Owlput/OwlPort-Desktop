<script setup>
import { invoke } from "@tauri-apps/api";
import { ref, onUnmounted } from "vue";
const mdns = ref({ discovered: 0 });
const kad = ref({ mode: false });
const autonat = ref({ status: "Unknown", confidence: 0 });
const upnp = ref({ status: false, num_of_exposed: 0 });
function update_display() {
  invoke("plugin:owlnest-mdns|list_discovered").then(
    (v) => (mdns.value.discovered = Object.keys(v).length)
  );
  invoke("plugin:owlnest-autonat|get_nat_status").then((result) => {
    autonat.value = result;
  });
  invoke("plugin:owlnest-kad|get_mode").then((result)=>kad.value.mode = result)
}
update_display();
let interval_id = setInterval(update_display,5000)
onUnmounted(()=>{
  clearInterval(interval_id)
})
</script>
<template>
  <main class="card-wrapper">
    <section
      class="shadow-md hover:shadow-lg relative w-40"
      @click="$router.push('/main/network/mdns')"
    >
      <p>mDNS</p>
      <div class="absolute bottom-0 w-full">
        <p>Discovered:</p>
        <p class="text-[3rem] p-4">{{ mdns.discovered }}</p>
      </div>
    </section>
    <section
      class="shadow-md hover:shadow-lg relative"
      @click="$router.push('/main/network/kad')"
    >
      <p class="text-xxl">Kad</p>
      <div class="w-full">
        <p>Mode: {{ kad.mode ? "Server" : "Client" }}</p>
      </div>
    </section>
    <section
      class="shadow-md hover:shadow-lg relative"
      @click="$router.push('/main/network/autonat')"
    >
      <p>AutoNat</p>
      <div class="absolute w-full bottom-0">
        <p>State: {{ autonat.status }}</p>
        <p>Confidence:</p>
        <p class="text-[3rem] p-4">{{ autonat.confidence }}</p>
      </div>
    </section>
    <section
      class="shadow-md hover:shadow-lg"
      @click="$router.push('/main/network/upnp')"
    >
      <p>UPnP</p>
      <div>
        <p>Status: {{ upnp.status ? "Available" : "Private" }}</p>
        <p v-if="upnp.status">Number of address requested:</p>
        <p v-if="upnp.status">{{ upnp.num_of_exposed }}</p>
      </div>
    </section>
  </main>
</template>
