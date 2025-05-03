<script setup lang="ts">
import { ref, watch, } from "vue";
import ChatList from "./components/ChatList.vue"
import { useRoute } from "vue-router";

/* 
  The window that contains both chat history(per peer) and a input area.
*/

const route = useRoute();
let remote = route.params.peerId as string

const focused_chat = ref(remote ? remote : "");

watch(() => route.params.peerId, () => {
    focused_chat.value = route.params.peerId as string;
})


</script>

<template>
    <main id="chat-layout-grid">
        <section class="border-r border-gray-300 w-full">
            <ChatList v-model:focused_chat="focused_chat" />
        </section>
        <section v-if="route.params.peerId" id="chat-container" class="h-[100vh]">
            <RouterView v-slot="{ Component }">
                <KeepAlive>
                    <component :is="Component" :key="route.params.peerId" />
                </KeepAlive>
            </RouterView>
        </section>
        <section v-else>No Chat</section>
    </main>
</template>
<style>
#chat-layout-grid {
    display: grid;
    grid-template-columns: 12rem auto;
    height: 100vh;
}
</style>