<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { is_ip_private, isBodylessHandler } from "../../utils";
import EntryExpand from "../../components/EntryExpand.vue";

const listen_addr = ref("/ip4/0.0.0.0/tcp/0");
const active_listeners: Ref<Array<string>> = ref([]);
function update_listener_list() {
  invoke<Array<string>>("plugin:owlnest-swarm|list_listeners")
    .then((result) => {
      active_listeners.value = result;
    })
    .catch(isBodylessHandler);
}
function remove_listener(address: string) {
  invoke("plugin:owlnest-swarm|remove_listener", { address });
  setTimeout(update_listener_list, 100)
}
update_listener_list();
function listen(addr?: string) {
  if ((!listen_addr.value || listen_addr.value.length < 1) && !addr) return;
  invoke("plugin:owlnest-swarm|listen", {
    listenOptions: { addr: addr ? addr : listen_addr.value },
  }).catch(isBodylessHandler);
  setTimeout(update_listener_list, 100);
}
</script>

<template>
  <section class="px-8 pt-8 pb-2">
    <form class="flex flex-row gap-4" @submit="() => listen()">
      <v-text-field class="text-xl" v-model="listen_addr" @keypress.enter="() => listen()"
        placeholder="Listen on an address, in MultiAddr" />
      <v-btn type="submit" size="x-large">Listen</v-btn>
    </form>
    <v-btn block
      @click="() => { listen('/ip4/0.0.0.0/tcp/0'); listen('/ip4/0.0.0.0/udp/0/quic-v1'); listen('/ip6/::/tcp/0'); listen('/ip6/::/udp/0/quic-v1') }">Listen
      on all addresses</v-btn>
  </section>
  <v-divider />
  <section>
    <header class="flex justify-between items-center select-none">
      <p class="text-lg px-8 py-2 w-fit">Active Listeners:</p>
      <button class="float-right h-[26px] mr-8" @click="update_listener_list">
        <span class="material-icons">refresh</span>
      </button>
    </header>
    <p class="shadow-md h-10 p-1 mx-8 text-center text-lg border border-gray-100 bg-blue-100"
      v-if="active_listeners.length < 1">No
      active listeners.</p>
    <ul v-else style="height: calc(100vh - 15rem - 1px); overflow: auto" class="select-none px-8">
      <li v-for="addr in active_listeners" class="my-1 shadow-md w-full h-10 p-0">
        <v-card :color="is_ip_private(addr.split('/')[2]) ? '#d4ffd5' : '#dbeafe'">
          <EntryExpand>
            <template #main>
              <p class=" w-full h-full p-2">{{ addr }}</p>
            </template>
            <template #more>
              <div class="flex gap-1 h-full p-1">
                <button @click="() => remove_listener(addr)"><span
                    class="material-icons icon-center">delete_forever</span></button>
                <button class="" @click="writeText(addr.toString())">
                  <span class="material-icons icon-center">content_copy</span></button>
              </div>
            </template>
          </EntryExpand>
        </v-card>
      </li>
    </ul>
  </section>
</template>