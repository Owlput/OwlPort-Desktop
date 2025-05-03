<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, Ref } from 'vue';
import { isBodylessHandler } from '../../../../utils';

const props = defineProps({
    focused_chat: {
        type: String,
        required: true
    }
})

const peer_search: Ref<String> = ref("");
const chats: Ref<Array<String>> = ref([]);
const loaded = ref(false);

const emit = defineEmits(["update:history"]);
const focused_chat = computed({
    get() {
        return props.focused_chat;
    },
    set(value) {
        emit("update:history", value);
    },
});


const filtered_chats = computed(() => {
    if (peer_search.value === "") return chats.value;
    else return chats.value.filter((v) => v.includes(peer_search.value.valueOf()));
});

invoke<Array<String>>("plugin:owlnest-messaging|get_all_chats")
    .then((v) => {
        chats.value = v;
        loaded.value = true;
    })
    .catch(isBodylessHandler)

</script>

<template>
    <v-text-field v-model="peer_search" class="w-full rounded-none" placeholder="Search for a peer..." density="compact"
        hide-details="true" />
    <ul class="flex flex-col">
        <li v-if="filtered_chats.length < 1 && chats.length !== 0">No chat fits your criteria</li>
        <li v-if="chats.length === 0 && loaded">
            <p class="text-slate-400 break-before-auto p-2">Only peers that had interactions with will be shown
                here
            </p>
        </li>
        <li v-for="chat in chats" class="min-w-[2rem]">
            <button class="w-full" :style="chat == focused_chat ? { backgroundColor: '#ddd' } : {}" @click="() => $router.push(`/app/messaging/chat/${chat}`)
            ">
                {{ chat.slice(-15, -1) }}
            </button>
        </li>
    </ul>
</template>