<script setup>
import { writeText } from "@tauri-apps/api/clipboard";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
const props = defineProps({
  peerId: String,
});
const pending_disconnect = ref(null);
const show_supported_protocols = ref(false);
const peer_info = ref({});
const connection_type = ["IPv4", "TCP", "Mocked"];
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
function on_disconnect() {
  if (!pending_disconnect.value) {
    pending_disconnect.value = setTimeout(
      () =>
        invoke("plugin:owlnest-swarm|disconnect_peer", {
          peerId: props.peerId,
        }),
      2000
    ); // Won't reach here when there is no backend
    return;
  }
  clearTimeout(pending_disconnect.value);
  pending_disconnect.value = null;
}
</script>
<template>
  <section
    @click.prevent.self="toggleExpand"
    class="flex flex-nowrap flex-row justify-between cursor-pointer"
  >
    <section>
      <span class="material-icons text-[3rem] w-full text-center"
        >computer</span
      >
      <p
        class="font-mono cursor-default"
        @dblclick="() => writeText(props.peerId)"
      >
        {{ props.peerId.slice(0, 6) }}..{{
          props.peerId.slice(props.peerId.length - 6, props.peerId.length)
        }}
      </p>
    </section>
    <section class="w-[16rem]">
      <ul class="flex flex-row gap-2">
        <li v-for="item in connection_type">
          <p>{{ item }}</p>
        </li>
      </ul>
      <section class="w-full mx-auto">
        <button class="bg-transparent" @click="on_disconnect">
          <img v-if="pending_disconnect" src="../../assets/unplug.svg" />
          <img v-else src="../../assets/plugged.svg" />
        </button>
      </section>
      <section>RTT: {{ "MOCKED" }} ms</section>
    </section>
  </section>
  <section class="mx-1" v-if="show_supported_protocols">
    <p class="sm:hidden">Peer ID: {{ props.peerId }}</p>
    <p>Protocol stack: {{ peer_info.protocol_version }}</p>
    <p>Suported protocols({{ peer_info.supported_protocols.length }}):</p>
    <ul class="flex flex-wrap">
      <li v-for="item in peer_info.supported_protocols" class="m-1">
        {{ item }}
      </li>
    </ul>
  </section>
</template>
