<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, Ref, computed, PropType } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { is_ip_private } from "../../utils";
import { RelayInfo, RelayStub } from "./types";
import AddressDisplay from "../../components/AddressDisplay.vue"
import SearchDisplay from "../../components/SearchDisplay.vue";
import { emit } from "@tauri-apps/api/event";
import { Popup } from "../../components/Types";

const props = defineProps({
  relay_stub: {
    type: Object as PropType<RelayStub>,
    required: true
  }
});
const relay_info: Ref<RelayInfo | null> = ref(null);
const show_relay_info = ref(false);
const advertise_throttle = ref(false);
const advertised = computed(() => {
  return relay_info.value?.advertised?.includes(
    localStorage.getItem("local_peer_id")!
  );
});

function toggle_relay_info() {
  if (show_relay_info.value) {
    show_relay_info.value = false;
    relay_info.value = null;
    return;
  }
  invoke<RelayInfo>("plugin:owlnest-relay|get_relay_info", {
    relay: props.relay_stub.peer_id,
  })
    .then((v) => {
      if (!v) return;
      v.listenable_address = v.listenable_address.filter(
        (v) => !(is_ip_private(v) || v.includes("wss"))
      );
      relay_info.value = v;
      invoke<Array<String> | null>("plugin:owlnest-advertise|query_advertised", {
        peerId: props.relay_stub.peer_id,
      })
        .then((v) => {
          if (!v) return;
          relay_info.value!.advertised = v
        }).catch((_e) => { }).finally(() => { show_relay_info.value = true });
    })

}
function listen_on_relay(address: String) {
  if (!relay_info.value || !address) return;
  invoke("plugin:owlnest-relay|listen_relay", {
    relayAddress: address + "/p2p/" + props.relay_stub.peer_id + "/p2p-circuit"
  }).then(() => {
    emit("newPopup", new Popup(Date.now(), "DefaultPopup", { message: "Successfully listening on relay" }))
    invoke<RelayInfo>("plugin:owlnest-relay|get_relay_info", {
      relay: props.relay_stub.peer_id,
    })
      .then((v) => {
        v.listenable_address = v.listenable_address.filter(
          (v) => !(is_ip_private(v) || v.includes("wss"))
        );
        relay_info.value!.listenable_address = v.listenable_address;
      })
      ;
  });
}

function dial_relayed(peer: String) {
  invoke("plugin:owlnest-swarm|dial", {
    dialOptions: { address: `/p2p/${props.relay_stub.peer_id}/p2p-circuit/p2p/${peer}` },
  });
}

function toggle_advertise() {
  advertise_throttle.value = true;
  invoke("plugin:owlnest-advertise|set_remote_advertisement", {
    remote: props.relay_stub.peer_id,
    advertisementState: !advertised.value,
  })
    .then(() => {
      setTimeout(
        () =>
          invoke<Array<String> | null>("plugin:owlnest-advertise|query_advertised", {
            peerId: props.relay_stub.peer_id,
          })
            .then((v) => {
              relay_info.value!.advertised = v;
            })
            .finally(() => (advertise_throttle.value = false)),
        100
      );
    })
    ;
}
function filter_advertised(source: String, search_text: String): boolean {
  return source.includes(search_text.valueOf())
}
async function get_relay_advertised_peers() {
  return relay_info.value!.advertised!
}
</script>
<template>
  <div class="flex justify-between flex-nowrap border px-2 cursor-pointer"
    @click.prevent.self="() => toggle_relay_info()">
    <p class="sm:hidden hover:cursor-pointer" @dblclick.prevent="writeText(props.relay_stub.peer_id.valueOf())">
      {{ props.relay_stub.peer_id.slice(0, 6) }}..{{
        props.relay_stub.peer_id.slice(props.relay_stub.peer_id.length - 6, props.relay_stub.peer_id.length)
      }}
    </p>
    <p class="hidden sm:block hover:cursor-pointer">
      {{ props.relay_stub.peer_id }}
    </p>
    <p>RTT: {{ props.relay_stub.latency }}ms</p>
  </div>

  <section v-if="show_relay_info" class="mx-2 border-x text-autowrap border-b">
    <section v-if="relay_info">
      <div class="h-6">
        <p class="px-2 float-left">
          Listenable addresses({{ relay_info.listenable_address.length }}):
        </p>
        <p v-if="relay_info.listenable_address.length !== 0" class="float-right mr-2 text-gray-500">
          Double-click on public address(blue) to listen on
        </p>
        <p v-else class="float-right mr-2 text-gray-500">No more address to listen on.</p>
      </div>
      <ul class="p-2 flex flex-wrap gap-2">
        <li v-for="addr in relay_info.listenable_address" class="max-w-[100%] hover:cursor-pointer"
          @dblclick="() => listen_on_relay(addr)">
          <AddressDisplay :address="addr.valueOf()" />
        </li>
      </ul>
    </section>
    <p v-else>Fetching info from backend...</p>
    <section v-if="relay_info?.advertised">
      <div class="h-6">
        <p class="hover:cursor-pointer px-2 float-left">
          Advertised peers({{ relay_info.advertised.length }}):
        </p>
        <button class="float-right px-1 mr-2" :disabled="advertise_throttle" @click="() => toggle_advertise()">
          {{ advertised ? "Remove Advertise" : "Advertise Self" }}
        </button>
      </div>
      <p v-if="relay_info.advertised.length === 0" class="px-4">No advertised peers</p>
      <SearchDisplay v-else :criteria="filter_advertised" :get-or-refresh="get_relay_advertised_peers"
        :min-item-size="20" scroller-height-expr="10rem" v-slot="slotProps"
        place-holder="Search for a peer. Press enter to commit search.">
        <li v-for="peerId in slotProps" class="my-0 w-full">
          <p class="sm:hidden hover:cursor-pointer" @dblclick="dial_relayed(peerId)">
            {{ peerId.slice(0, 6) }}..{{
              peerId.slice(peerId.length - 6, peerId.length)
            }}
          </p>
          <p class="hidden sm:block hover:cursor-pointer">
            {{ peerId }}
          </p>
        </li>
      </SearchDisplay>
      <ul class="px-4">

      </ul>
    </section>
  </section>
</template>
