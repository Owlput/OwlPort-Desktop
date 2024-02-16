<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
const props = defineProps({
  peerId: String,
});
const show_supported_protocols = ref(false);
const peer_info = ref({});
function toggleExpand() {
  if (show_supported_protocols.value === false) {
    invoke("plugin:owlnest-swarm|get_peer_info", { peerId: props.peerId }).then(
      (v) => {
        peer_info.value = v;
        show_supported_protocols.value = true;
      }
    );
  } else {
    show_supported_protocols.value = false;
  }
}
</script>
<template>
  <section class="flex flex-nowrap flex-row justify-between hover:cursor-pointer">
    <p @click="toggleExpand">{{ props.peerId }}</p>
    <button class="rounded-sm"
      @dblclick="
        () =>
          invoke('plugin:owlnest-swarm|disconnect_peer', {
            peerId: props.peerId,
          })
      "
    >
      Disconnect
    </button>
  </section>
  <section class="mx-1" v-if="show_supported_protocols">
    <p>Protocol stack: {{ peer_info.protocol_version }}</p>
    <p>Suported protocols({{ peer_info.supported_protocols.length }}):</p>
    <ul class="flex flex-wrap">
      <li v-for="item in peer_info.supported_protocols" class="m-2">
        {{ item }}
      </li>
    </ul>
  </section>
</template>
