<script setup lang="ts">
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { invoke } from "@tauri-apps/api/core";
import { ref, Ref } from "vue";
import { isBodylessHandler } from "../../utils";
import * as types from "./types";

const props = defineProps({
  peerId: {
    type: String,
    required: true,
  },
});
const pending_disconnect: Ref<Number | null> = ref(null);
const show_supported_protocols = ref(false);
const peer_info: Ref<types.PeerInfo | null> = ref(null);
const connection_type = ["IPv4", "TCP", "Mocked"];
function toggleExpand() {
  if (show_supported_protocols.value === false) {
    invoke<any>(
      "plugin:owlnest-swarm|get_peer_info",
      {
        peerId: props.peerId,
      }
    )
      .then((v) => {
        peer_info.value = new types.PeerInfo(v[0].supported_protocols, v[0].protocol_version, v[1]);
        show_supported_protocols.value = true;
      })
      .catch(isBodylessHandler);
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
        }).catch(isBodylessHandler),
      2000
    ); // Won't reach here when there is no backend
    return;
  }
  clearTimeout(pending_disconnect.value.valueOf());
  pending_disconnect.value = null;
}


// function parse_protocol(proto: string): [string, string] {
//   if (proto.startsWith("/libp2p/autonat")) return ["AutoNAT", "Auto-discover of NAT status"];
//   if (proto.startsWith("/ipfs/ping")) return ["Ping", "Ping protocol for universal connectivity"];
//   if (proto.startsWith("/ipfs/id")) return ["Identify", "Identify protocol for universal connectivity"];
//   if (proto.startsWith("/owlnest/messaging")) return ["OwlNest Messaging","Direct messaging"]
//   if (proto.startsWith("/owlnest/blob")) return ["OwlNest Blob Transfer","Direct transfer of bulk binary data"];
//   if (proto.startsWith("/libp2p/circuit/relay"))
//   return [proto, ""]
// }
</script>
<template>
  <div class="border rounded-sm shadow-md mx-4 px-4 select-none">
    <section @click.prevent.self="toggleExpand" class="flex flex-nowrap flex-row justify-between cursor-pointer">
      <section>
        <span class="material-icons text-[3rem] w-full text-center">computer</span>
        <p class="font-mono cursor-default" @dblclick="() => writeText(props.peerId)">
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
    <hr v-if="show_supported_protocols" />
    <section v-if="show_supported_protocols && peer_info">
      <p class="sm:hidden">Peer ID: {{ props.peerId }}</p>
      <p>Protocol stack: {{ peer_info.protocol_version }}</p>
      <p>Suported protocols({{ peer_info.supported_protocols.length }}):</p>
      <ul class="flex flex-wrap">
        <li v-for="item in peer_info.supported_protocols" class="m-1 border px-1">
          {{ item }}
        </li>
      </ul>
    </section>
    <section v-if="!peer_info && show_supported_protocols">Fetching peer info from backend...</section>
  </div>
</template>
