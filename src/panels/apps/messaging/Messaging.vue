<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, Ref, onUnmounted } from "vue";
import { RouterView, useRouter } from "vue-router";
import { isBodylessHandler } from "../../../utils";
import PopUpHandler from "../../../components/PopUpHandler.vue";
import SideBar from "./SideBar.vue";

defineOptions({
  name: "Messaging",
});
let router = useRouter();
const chats: Ref<Array<String>> = ref([]);
const focus_chat_rlisten: Ref<Function> = ref(() => { });

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
  <section class="grid h-[100vh]" id="messaging-layout-grid">
    <SideBar />
    <section>
      <RouterView v-slot="{ Component }">
        <component :is="Component" />
      </RouterView>
    </section>
  </section>
  <PopUpHandler />
</template>
<style scoped>
#messaging-layout-grid {
  grid-template-columns: 5rem auto;
}
</style>
