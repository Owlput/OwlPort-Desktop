<script setup>
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const address_to_probe = ref("");
const nat_status = ref({ confidence: 0, status: "Unknown" });
function get_nat_status() {
  invoke("plugin:owlnest-autonat|get_nat_status").then(
    (result) => (nat_status.value = result)
  );
}
get_nat_status();
function probe() {
  if (!address_to_probe) {
    return;
  }
  invoke("plugin:owlnest-autonat|probe", {
    address: address_to_probe.value,
  }).catch((e) => console.log(e));
  setTimeout(get_nat_status, 500);
}
</script>

<template>
  <div class="px-8 py-4">
    <section class="single-input">
      <input v-model="address_to_probe" @keydown.enter.exact.prevent="probe" />
      <button @click="probe">Probe</button>
    </section>
  </div>
  <section class="px-8 py-4">
    <p>Current NAT status: {{ nat_status.status }}</p>
    <p>Confidence Score: {{ nat_status.confidence }}</p>
  </section>
</template>
