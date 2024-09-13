<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, Ref, computed } from "vue";
import { writeText } from "@tauri-apps/api/clipboard";
import { isBodylessHandler } from "../../utils";
import { RelayInfo } from "./types";
import AddressDisplay from "../../components/AddressDisplay.vue"

const props = defineProps<{
  peerId: String;
  latency: Number;
}>();
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
    relay: props.peerId,
  })
    .then((v) => {
      v.address = v.address.filter(
        (v) => !(v.includes("127.0.0.1") || v.includes("wss"))
      );
      relay_info.value = v;
      invoke<Array<String> | null>("plugin:owlnest-advertise|query_advertised", {
        peerId: props.peerId,
      })
        .then((v) => {
          if (!v) return;
          relay_info.value!.advertised = v
        })
        .catch((e) => {
          isBodylessHandler(e);

        });
    })
    .catch(isBodylessHandler).finally(() => show_relay_info.value = true);

}
function listen_on_relay() {
  if (!relay_info.value) return;
  invoke("plugin:owlnest-swarm|listen", {
    listenOptions: {
      addr: `${relay_info.value.address[0]}/p2p/${props.peerId}/p2p-circuit`,
    },
  }).catch(isBodylessHandler);
}

function dial_relayed(peer: String) {
  invoke("plugin:owlnest-swarm|dial", {
    dialOptions: { address: `/p2p/${props.peerId}/p2p-circuit/p2p/${peer}` },
  }).catch(isBodylessHandler);
}

function toggle_advertise() {
  advertise_throttle.value = true;
  invoke("plugin:owlnest-advertise|set_remote_advertisement", {
    remote: props.peerId,
    advertisementState: !advertised.value,
  })
    .then(() => {
      setTimeout(
        () =>
          invoke<Array<String>|null>("plugin:owlnest-advertise|query_advertised", {
            peerId: props.peerId,
          })
            .then((v) => {
              console.log(v);
              relay_info.value!.advertised = v;
            })
            .finally(() => (advertise_throttle.value = false)),
        100
      );
    })
    .catch(isBodylessHandler);
}
</script>
<template>
  <div class="flex justify-between flex-nowrap border px-2 cursor-pointer"
    @click.prevent.self="() => toggle_relay_info()">
    <p class="select-none font-mono cursor-default"
      @dblclick.prevent="writeText(props.peerId.valueOf()).catch(isBodylessHandler)">
      {{ props.peerId }}
    </p>
    <p>RTT: {{ props.latency }}ms</p>
  </div>

  <section v-if="show_relay_info" class="mx-2 border-x text-autowrap border-b">
    <section v-if="relay_info">
      <div class="h-6">
        <p class="px-2 float-left">
          Listenable addresses({{ relay_info.address.length }}):
        </p>
        <button class="float-right mr-2" v-if="relay_info.address.length != 0" @click="() => listen_on_relay()">
          Listen
        </button>
      </div>
      <ul class="p-2 flex flex-wrap gap-2">
        <li v-if="relay_info.address.length === 0">
          No listenable address(Addresses not made public)
        </li>
        <li v-for="addr in relay_info.address">
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
      <ul class="px-4">
        <li v-if="relay_info.advertised.length === 0">No advertised peers</li>
        <li v-for="peerId in relay_info.advertised" class="my-0 w-full">
          <p class="sm:hidden hover:cursor-pointer" @dblclick="dial_relayed(peerId)">
            {{ peerId.slice(0, 6) }}..{{
              peerId.slice(peerId.length - 6, peerId.length)
            }}
          </p>
          <p class="hidden sm:block hover:cursor-pointer">
            {{ peerId }}
          </p>
        </li>
      </ul>
    </section>
  </section>
</template>
