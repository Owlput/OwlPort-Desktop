<script setup>
import { invoke } from "@tauri-apps/api";
import { ref, computed } from "vue";
import { writeText } from "@tauri-apps/api/clipboard";
const props = defineProps({
  peerId: String,
  latency: Number,
});
const relay_info = ref({});
const show_relay_info = ref(false);
const advertise_throttle = ref(false);
const advertised = computed(() => {
  return relay_info.value.advertised.includes(
    localStorage.getItem("local_peer_id")
  );
});

function toggle_relay_info() {
  if (!show_relay_info.value) {
    invoke("plugin:owlnest-relay|get_relay_status", {
      relay: props.peerId,
    })
      .then((v) => {
        v.address = v.address.filter(
          (v) => !(v.includes("127.0.0.1") || v.includes("wss"))
        );
        relay_info.value = v;
        invoke("plugin:owlnest-advertise|query_advertised", {
          peerId: props.peerId,
        })
          .then((v) => {
            (relay_info.value.advertised = v), (show_relay_info.value = true);
          })
          .catch((e) => {
            show_relay_info.value = true;
          });
      })
      .catch((_) => {});
  } else {
    show_relay_info.value = false;
    relay_info.value = false;
  }
}
function listen_on_relay() {
  invoke("plugin:owlnest-swarm|listen", {
    listenOptions: {
      addr: `${relay_info.value.address[0]}/p2p/${props.peerId}/p2p-circuit`,
    },
  }).catch((e) => console.log(e));
}

function dial_relayed(peer) {
  invoke("plugin:owlnest-swarm|dial", {
    dialOptions: { address: `/p2p/${props.peerId}/p2p-circuit/p2p/${peer}` },
  });
}

function toggle_advertise() {
  advertise_throttle.value = true;
  invoke("plugin:owlnest-advertise|set_remote_advertisement", {
    remote: props.peerId,
    advertisementState: !advertised.value,
  }).then(() => {
    setTimeout(
      () =>
        invoke("plugin:owlnest-advertise|query_advertised", {
          peerId: props.peerId,
        })
          .then((v) => {
            console.log(v);
            relay_info.value.advertised = v;
          })
          .catch((e) => {
            console.log(e);
          })
          .finally(() => (advertise_throttle.value = false)),
      100
    );
  });
}
</script>
<template>
  <div
    class="flex justify-between flex-nowrap border px-2 cursor-pointer"
    @click.prevent.self="() => toggle_relay_info()"
  >
    <p
      class="select-none font-mono cursor-default"
      @dblclick.prevent="writeText(props.peerId)"
    >
      {{ props.peerId }}
    </p>
    <p>RTT: {{ props.latency }}ms</p>
  </div>

  <section v-if="show_relay_info" class="mx-2 border-x text-autowrap border-b">
    <section>
      <div class="h-6">
        <p class="px-2 float-left">
          Listenable addresses({{ relay_info.address.length }}):
        </p>
        <button
          class="float-right mr-2"
          v-if="relay_info.address.length != 0"
          @click="() => listen_on_relay()"
        >
          Listen
        </button>
      </div>

      <ul class="px-4">
        <li v-if="relay_info.address.length === 0">
          No listenable address(Addresses not public)
        </li>
        <li v-for="addr in relay_info.address" class="my-0 w-full">
          <p>
            {{ addr }}
          </p>
        </li>
      </ul>
    </section>
    <section v-if="relay_info.advertised">
      <div class="h-6">
        <p class="hover:cursor-pointer px-2 float-left">
          Advertised peers({{ relay_info.advertised.length }}):
        </p>
        <button
          class="float-right px-1 mr-2"
          :disabled="advertise_throttle"
          @click="() => toggle_advertise()"
        >
          {{ advertised ? "Remove Advertise" : "Advertise Self" }}
        </button>
      </div>
      <ul class="px-4">
        <li v-if="relay_info.advertised.length === 0">No advertised peers</li>
        <li v-for="peerId in relay_info.advertised" class="my-0 w-full">
          <p
            class="sm:hidden hover:cursor-pointer"
            @dblclick="dial_relayed(peerId)"
          >
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
