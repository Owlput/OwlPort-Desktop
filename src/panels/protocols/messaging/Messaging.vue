<script setup>
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { ref, computed, onUnmounted } from "vue";
import { RouterView, useRouter } from "vue-router";
defineOptions({
  name: "Messaging",
});
let router = useRouter();
const chats = ref([]);
const focus_chat_rlisten = ref("");
const filtered_chats = computed(() => {
  if (peer_search === "") return chats.value;
  else return chats.value.filter((v) => v.includes(peer_search.value));
});
const peer_search = ref("");
function handle_focus_chat(ev) {
  if (!chats.value.includes(ev?.payload))
    invoke("plugin:owlnest-messaging|get_all_chats").then(
      (v) => (chats.value = v)
    );
    router.push(`/app/messaging/${ev.payload}`)
}
listen("focusChat", handle_focus_chat).then((handle) => {
  focus_chat_rlisten.value = handle;
});
invoke("plugin:owlnest-messaging|get_all_chats").then(
      (v) => (chats.value = v)
    );
onUnmounted(() => {
  focus_chat_rlisten.value();
});
</script>

<template>
  <main class="grid" id="messaging-grid">
    <section class="border-r border-b w-full">
      <input
        v-model="peer_search"
        class="w-full rounded-none"
        placeholder="Search for a peer..."
      />
      <ul class="flex flex-col">
        <li v-if="filtered_chats.length < 1">No chat fits your criteria</li>
        <li v-for="chat in chats" class="min-w-[2rem]">
          <button
            class="w-full"
            :style="chat == $route.params.id ? { backgroundColor: '#ddd' } : {}"
            @click="
              () => {
                $router.push(`/app/messaging/${chat}`);
              }
            "
          >
            {{ chat.slice(-15, -1) }}
          </button>
        </li>
      </ul>
    </section>
    <section>
      <RouterView v-slot="{ Component }"
        ><KeepAlive>
          <component :is="Component" :key="$route.params.id"
        /></KeepAlive>
      </RouterView>
    </section>
  </main>
</template>
<style scoped>
#messaging-grid {
  grid-template-columns: 10rem auto;
}
</style>
