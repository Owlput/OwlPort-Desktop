<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import ChatInput from './ChatInput.vue';
import MessageHistory from './MessageHistory.vue';
import { isBodylessHandler } from '../../../../utils';
import { nextTick, Ref, ref } from 'vue';
import { Message } from '../types';
import { useRoute } from 'vue-router';

const route = useRoute();

const remote = route.params.peerId as string;

const history: Ref<Array<Message>> = ref([]);


function send(target: String, msg: String) {
    invoke("plugin:owlnest-messaging|send_msg", { target, msg }).catch(
        isBodylessHandler
    );
}

function push_history(message: Message) {
    history.value.push(message);
    nextTick(() => {
        let element = document.getElementById(`chat-history`);
        if (!element) {
            console.error("Internal state error: no 'chat-history' to select");
            return;
        }
        console.log(element.scrollHeight - element.scrollTop)
        if ((element.scrollHeight - element.scrollTop - window.innerHeight - 300) < 200) {
            document.querySelector("#chat-history > li:last-child")?.scrollIntoView();
        }
    });
}

function clear_history() {
    invoke('plugin:owlnest-messaging|clear_chat_history', {
        peerId: remote,
    });
    history.value = [];
}

</script>

<template>
    <div class="flex flex-col">
        <div class="h-[60%] w-full">
            <MessageHistory v-model:history="history" :remote="remote" />
        </div>
        <div class="h-[40%] w-full">
            <ChatInput :send_message="send" :push_history="push_history" :clear_history="clear_history"
                :remote="remote" />
        </div>
    </div>
</template>