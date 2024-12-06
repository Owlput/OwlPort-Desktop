<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, Ref, computed, onUnmounted } from "vue";
import { RouterView, useRouter } from "vue-router";
import { isBodylessHandler } from "../../../utils";
import PopUpHandler from "../../../components/PopUpHandler.vue";

defineOptions({
  name: "Messaging",
});
let router = useRouter();
const chats: Ref<Array<String>> = ref([]);
const focus_chat_rlisten: Ref<Function> = ref(() => { });
const peer_search: Ref<String> = ref("");

const filtered_chats = computed(() => {
  if (peer_search.value === "") return chats.value;
  else return chats.value.filter((v) => v.includes(peer_search.value.valueOf()));
});

function handle_focus_chat(ev: any) {
  if (!chats.value.includes(ev.payload!))
    invoke<Array<String>>("plugin:owlnest-messaging|get_all_chats")
      .then((v) => (chats.value = v))
      .catch(isBodylessHandler);
  router.push(`/app/messaging/${ev.payload}`);
}
listen("focusChat", handle_focus_chat).then((handle) => {
  focus_chat_rlisten.value = handle;
}).catch(isBodylessHandler);
invoke<Array<String>>("plugin:owlnest-messaging|get_all_chats")
  .then((v) => (chats.value = v))
  .catch(isBodylessHandler);
onUnmounted(() => {
  focus_chat_rlisten.value();
});
</script>

<template>
  <main class="grid h-[100vh]" id="messaging-grid">
    <section class="border-r border-b w-full">
      <input v-model="peer_search" class="w-full rounded-none" placeholder="Search for a peer..." />
      <ul class="flex flex-col p-1">
        <li v-if="filtered_chats.length < 1 && chats.length !== 0">No chat fits your criteria</li>
        <li v-if="chats.length === 0">
          <p class="text-slate-400 break-before-auto p-2">Only peers that had interactions with will be shown here</p>
        </li>
        <li v-for="chat in chats" class="min-w-[2rem]">
          <button class="w-full" :style="chat == $route.params.id ? { backgroundColor: '#ddd' } : {}" @click="() => $router.push(`/app/messaging/${chat}`)
            ">
            {{ chat.slice(-15, -1) }}
          </button>
        </li>
      </ul>
    </section>
    <section>
      <RouterView v-slot="{ Component }">
        <KeepAlive>
          <component :is="Component" :key="$route.params.id" />
        </KeepAlive>
      </RouterView>
    </section>
  </main>
  <PopUpHandler />
</template>
<style scoped>
#messaging-grid {
  grid-template-columns: 10rem auto;
}
</style>
