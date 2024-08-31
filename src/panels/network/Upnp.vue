<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { isBodylessHandler } from "../../utils";

const public_address = ref([]);
const gateway_status = ref("");
invoke("plugin:owlnest-upnp|list_available_external_addr")
  .then((result) => (public_address.value = result))
  .catch(isBodylessHandler);
invoke("plugin:owlnest-upnp|get_gateway_status")
  .then((result) => {
    if (result === -1) {
      gateway_status.value = "Gateway not found or doesn't support UPnP";
    } else if (result === 0) {
      gateway_status.value = "Gateway not publicly reachable";
    } else if (result === 1) {
      gateway_status.value = "Gateway publicly reachable";
    } else {
      console.error("UPnP: unhandled gateway status");
    }
  })
  .catch(isBodylessHandler);
</script>
<template>
  <section class="px-8 py-4">
    <p>Gateway status: {{ gateway_status }}</p>
    <section>
      <p>
        Public addresses:
        {{ public_address.length > 0 ? "" : "No public address" }}
      </p>
      <ul>
        <li v-for="addr in public_address">{{ addr }}</li>
      </ul>
    </section>
  </section>
</template>
