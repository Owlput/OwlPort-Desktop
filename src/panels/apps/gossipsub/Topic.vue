<script setup lang="ts">
import { ref, Ref } from 'vue';
import { ReadableTopic, TopicInfo } from './types';
import { invoke } from '@tauri-apps/api';

const props = defineProps({
    topic: {
        type: ReadableTopic,
        required: true
    }
});
const topic: Ref<TopicInfo | null> = ref(null);
// invoke<TopicInfo>("plugin:libp2p-gossipsub|get_topic_info", { hash: props.hash }).then(
//     (v) => topic.value = v
// )
</script>
<template>
    <section v-if="props.topic">
        <section v-if="!props.topic.StringOnly" class="flex justify-between items-center">
            <p >Hash: {{ props.topic.get_hash()! }}</p>
            <button class="h-7 aspect-square"><span class="material-icons">content_copy</span></button>
        </section>
        <section v-if="!props.topic.HashOnly" class="flex justify-between items-center">
            <p class="">String repr: {{ props.topic.get_string()! }}</p>
            <button class="h-7 aspect-square"><span class="material-icons">content_copy</span></button>
        </section>
    </section>
    <section v-else>
        <p>Loading topic info...</p>
    </section>
</template>