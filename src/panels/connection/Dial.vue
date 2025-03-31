<script setup lang="ts">
import { computed, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRoute } from "vue-router";
import { isBodylessHandler, REG_VALIDATE_IPV4, REG_VALIDATE_IPV6 } from "../../utils";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const route = useRoute();
defineOptions({
  name: "Dial",
});
const address_to_dial = ref(route.query?.dial ? route.query.dial as string : "");
const dial_history: Ref<Array<string>> = ref([]);
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
const ipQuickDial = ref({ version: 4, ip: "", protocol: "tcp", port: "" });
function apply_ip_quick_dial() {
  if (!ipQuickDial.value.version || ipQuickDial.value.ip.length === 0 || ipQuickDial.value.protocol === "" || ipQuickDial.value.port.length === 0) return;
  address_to_dial.value = `/ip${ipQuickDial.value.version}/${ipQuickDial.value.ip}/${ipQuickDial.value.protocol}/${ipQuickDial.value.port}`
}
const ipValidator = computed(() => {
  if (ipQuickDial.value.version === 4) {
    return REG_VALIDATE_IPV4.source
  }
  if (ipQuickDial.value.version === 6) {
    return REG_VALIDATE_IPV6.source
  }
  throw new Error("Invalid IP edition")
})
const dnsQuickDial = ref({ domain_name: "", peer_id: "" });
function apply_dns_quick_dial() {
  if (dnsQuickDial.value.domain_name.length === 0) return;
  if (!dnsQuickDial.value.peer_id) return address_to_dial.value = `/dns/${dnsQuickDial.value.domain_name}`;
  address_to_dial.value = `/dns/${dnsQuickDial.value.domain_name}/p2p/${dnsQuickDial.value.peer_id}`
} 
function update_dial_history(addr:string){
  let index = dial_history.value.findIndex((entry)=>entry === addr);
  if (index >= 0){
    let value = dial_history.value.splice(index,1);
    dial_history.value.unshift(value[0])
    return;
  }
  dial_history.value.unshift(addr)
}
</script>

<template>
  <section class="px-8 py-4 border-b">
    <p class="text-left w-full px-4 text-lg">Dial a peer</p>
    <div class="single-input">
      <input class="text-xl" v-model="address_to_dial" @keypress.enter.exact.prevent="() => dial()" />
      <button @click="dial" class="text-xl">Dial</button>
    </div>
  </section>
  <section style="border-bottom: 1px gainsboro solid;" class="p-4">
    <p>Quick Dial</p>
    <form class="flex flex-wrap justify-between my-1" @submit="apply_ip_quick_dial">
      <div class="w-[80%]">
        <input class="min-w-[8rem] w-[45%] py-1 px-2 mx-1" placeholder="IP address" type="text" required
          :pattern="ipValidator" v-model="ipQuickDial.ip" />
        <input class="min-w-[8rem] w-16 py-1 px-2 mx-1" placeholder="Port" type="number" required
          v-model="ipQuickDial.port" max="65536" />
      </div>
      <button type="submit" class="min-w-[4rem]" @click="dial">Dial</button>
      <button type="submit" class=""
        @click.prevent="writeText(`/ip${ipQuickDial.version}/${ipQuickDial.ip}/${ipQuickDial.protocol}/${ipQuickDial.port}`)">
        <span class="material-icons icon-center">content_copy</span></button>
    </form>
    <form class="flex flex-wrap justify-between my-1" @submit="apply_dns_quick_dial">
      <div class="w-[80%]">
        <input class="min-w-[8rem] w-[45%] py-1 px-2 mx-1" placeholder="Domain name" type="text" required
          v-model="dnsQuickDial.domain_name" />
        <input class="min-w-[8rem] w-[45%] py-1 px-2 mx-1" placeholder="Peer ID" type="text"
          v-model="dnsQuickDial.peer_id" />
      </div>
      <button type="submit" class="min-w-[4rem]" @click="dial">Dial</button>
      <button type="submit" class="aspect-square">
        <span class="material-icons icon-center">content_copy</span></button>
    </form>
    <p>Wrap Dial(Work In Progress)</p>
    <form class="flex flex-wrap justify-between">
      <div class="w-[80%]">
        <input class="min-w-[8rem] w-[80%] py-1 px-2" placeholder="Peer ID" type="text" />
      </div>
      <button type="submit" class="min-w-[4rem]">Dial</button>
    </form>
  </section>
  <section>
    <p class="px-4">Dial history</p>
    <ul class="px-4 overflow-auto" style="height: calc(100vh - 23rem);">
      <li v-for="addr in dial_history" class="border my-1 flex flex-row justify-between">
        <p class="p-2 w-fit">{{ addr }}</p>
        <div>
          <button class="m-1 float-left" @click="() => address_to_dial = addr">
            <span class="material-icons text-2xl icon-center">arrow_upward</span>
          </button>
          <button class="m-1 float-right" @click="() => writeText(addr)">
            <span class="material-icons text-2xl icon-center">content_copy</span>
          </button>
        </div>
      </li>
    </ul>
  </section>
</template>
