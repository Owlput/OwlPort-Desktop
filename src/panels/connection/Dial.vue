<script setup lang="ts">
import { Ref, ref, shallowRef } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRoute } from "vue-router";
import { isBodylessHandler, REG_VALIDATE_IPV4, REG_VALIDATE_IPV6 } from "../../utils";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const route = useRoute();
defineOptions({
  name: "Dial",
});

const address_to_dial = ref(route.query?.dial ? route.query.dial as string : "");
function dial() {
  if (!address_to_dial.value) return;
  setTimeout(
    () => {
      let address: string = address_to_dial.value;
      invoke("plugin:owlnest-swarm|dial", {
        dialOptions: { address },
      }).catch(isBodylessHandler).finally(() => update_dial_history(address))
    }, 5)
}

const ipQuickDial = ref({ ip: "", port: null });
const show_quick_dial = ref(true);
const transport_protocol = shallowRef( 10 );
const transport_protocol_items = [{ name: 'TCP', value: 10 }, { name: 'UDP-QUIC', value: 21 }];
const multiaddr_rules = {
  required: (value: any) => !!value || "Required",
  ip_addr: (value: string) => REG_VALIDATE_IPV4.exec(value) !== null || REG_VALIDATE_IPV6.exec(value) !== null || "Invalid IP(v4 and v6)",
  port: (value: number) => value >= 1 && value <= 65535 || "Invalid"
}
function apply_ip_quick_dial() {
  if (ipQuickDial.value.ip.length === 0 || ipQuickDial.value.port === null) return;
  let ip_version = REG_VALIDATE_IPV4.exec(ipQuickDial.value.ip) ? "/ip4" : REG_VALIDATE_IPV6.exec(ipQuickDial.value.ip) ? "/ip6" : null;
  if (ip_version === null) return;
  let transport;
  switch (Math.floor(transport_protocol.value / 10)) {
    case 1: transport = "tcp"; break;
    case 2: transport = "udp"; break;
    default: return;
  }
  let quic = "";
  switch (transport_protocol.value % 10) {
    case 0: break;
    case 1: quic = "/quic-v1"; break;
    default: return;
  }
  let address_to_dial = `${ip_version}/${ipQuickDial.value.ip}/${transport}/${ipQuickDial.value.port}${quic}`
  console.log(address_to_dial);
}
const dnsQuickDial = ref({ domain_name: "", peer_id: "" });
function apply_dns_quick_dial() {
  if (dnsQuickDial.value.domain_name.length === 0) return;
  if (!dnsQuickDial.value.peer_id) return address_to_dial.value = `/dns/${dnsQuickDial.value.domain_name}`;
  address_to_dial.value = `/dns/${dnsQuickDial.value.domain_name}/p2p/${dnsQuickDial.value.peer_id}`
}

const dial_history: Ref<Array<string>> = ref([]);
const reorder_history = ref(true);
function update_dial_history(addr: string) {
  let index = dial_history.value.findIndex((entry) => entry === addr);
  if (index >= 0) {
    // The address is present
    if (reorder_history.value) {
      // Move to front
      let value = dial_history.value.splice(index, 1);
      dial_history.value.unshift(value[0])
    }
    // Do nothing when position is locked and the address is present.
    return;
  }
  // The address is not present
  if (reorder_history.value) {
    // push to front
    dial_history.value.unshift(addr);
    return;
  }
  dial_history.value.push(addr)
}
</script>

<template>
  <v-form class="flex flex-row gap-4 px-8 pt-8">
    <v-text-field v-model="address_to_dial" @keypress.enter.exact.prevent="() => dial()"
      label="Dial a peer, using MultiAddr" />
    <v-btn @click="dial" size="x-large">Dial</v-btn>
  </v-form>
  <v-divider />
  <section class="px-8">
    <header class="h-8">
      <p class="float-left">Quick Dial</p>
      <button class="float-right cursor-pointer" @click="() => show_quick_dial = !show_quick_dial">
        <span class="mdi-chevron-up mdi text-xl block"
          :style="show_quick_dial ? 'rotate: 0deg;' : 'rotate:180deg;'"></span>
      </button>
    </header>
    <v-expand-transition>
      <main class="overflow-hidden" v-show="show_quick_dial">
        <form class="flex flex-row flex-wrap" @submit="apply_ip_quick_dial">
          <v-text-field class="min-w-[10rem] w-[35%] mx-1" label="IP address" type="text" required
            :rules="[multiaddr_rules.required, multiaddr_rules.ip_addr]" validate-on="submit"
            v-model="ipQuickDial.ip" />
          <v-select class="min-w-[9rem] max-w-[9rem] mx-1" label="Protocol" :items="transport_protocol_items"
            v-model="transport_protocol" item-title="name" item-value="value"></v-select>
          <v-text-field class="min-w-[7rem] max-w-[7rem] w-[16%] mx-1" label="Port" required
            :rules="[multiaddr_rules.required, multiaddr_rules.port]" v-model="ipQuickDial.port" />
          <v-btn class="mb-4 mx-1" type="submit" width="4rem" height="3.5rem" size="large">Dial</v-btn>
          <v-btn class="mb-4 mx-1" width="2rem" height="3.5rem" size="x-small"
            >
            <span class="material-icons icon-center">content_copy</span>
          </v-btn>
        </form>
        <form class="flex flex-wrap" @submit="apply_dns_quick_dial">
          <v-text-field class="min-w-[10rem] w-[32%] mx-1" placeholder="Domain name" type="text" required
            :rules="[multiaddr_rules.required]" v-model="dnsQuickDial.domain_name" validate-on="submit" />
          <v-text-field class="min-w-[10rem] w-[45%] mx-1" placeholder="Peer ID" type="text"
            v-model="dnsQuickDial.peer_id" />
          <v-btn class="mx-1" type="submit" width="4rem" height="3.5rem" size="large">Dial</v-btn>
          <v-btn class="mx-1" type="submit" width="2rem" height="3.5rem" size="x-small">
            <span class="mdi-content-copy mdi text-2xl" aria-hidden="true"></span>
          </v-btn>
        </form>
        <v-divider />
        <p class="my-2">Wrap Dial(Work In Progress)</p>
        <form class="flex flex-wrap gap-4">
          <v-text-field class="min-w-[8rem]" placeholder="Peer ID" type="text" />
          <v-btn type="submit" size="x-large">Dial</v-btn>
        </form>
      </main>
    </v-expand-transition>
  </section>
  <v-divider />
  <section>
    <header class="flex justify-between px-8">
      <p class="text-lg">Dial history</p>
      <div class="flex flex-row gap-2">
        <button class="cursor-pointer" @click="dial_history.reverse">
          <span class="mdi-swap-vertical mdi text-2xl"></span>
          <v-tooltip activator="parent" location="bottom">Reverse order</v-tooltip>
        </button>
        <button class="cursor-pointer" @click="() => reorder_history = !reorder_history">
          <span
            :class="(reorder_history ? 'mdi-sort-clock-ascending-outline' : 'mdi-lock-outline') + ' mdi text-2xl'"></span>
          <v-tooltip activator="parent" location="bottom">
            {{ reorder_history ? "Promote to front" : "Position locked" }}
          </v-tooltip>
        </button>
      </div>
    </header>
    <ul class="px-8 overflow-auto"
      :style="show_quick_dial ? 'height: calc(100vh - 31rem - 3px);' : 'height: calc(100vh - 14rem);'">
      <li v-for="addr in dial_history" class="flex flex-row justify-between mt-0.5 border border-gray-200">
        <p class="p-2 text-lg select-none cursor-pointer" @click="() => address_to_dial = addr">{{ addr }}</p>
        <div>
          <button class="cursor-pointer h-full" @click="() => writeText(addr)">
            <span class="mdi-content-copy mdi text-2xl" aria-hidden="true"></span>
          </button>
        </div>
      </li>
    </ul>
  </section>
</template>
