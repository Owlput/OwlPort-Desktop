<script setup lang="ts">
import { ref, Ref } from 'vue';
import { ReadableTopic, TopicInfo } from './types';
import { invoke } from '@tauri-apps/api';
import { isBodylessHandler } from '../../../utils';
import { writeText } from '@tauri-apps/api/clipboard';
import ToolTip from '../../../components/ToolTip.vue';

const props = defineProps({
    topic: {
        type: ReadableTopic,
        required: true
    },
    unsub: Boolean
});
const emit = defineEmits(["unsubscribe"])
const topic: Ref<TopicInfo | null> = ref(null);
// invoke<TopicInfo>("plugin:libp2p-gossipsub|get_topic_info", { hash: props.hash }).then(
//     (v) => topic.value = v
// )
function unsubscribe() {
    invoke("plugin:libp2p-gossipsub|unsubscribe", { topic: props.topic })
        .then((_v) => emit('unsubscribe'))
        .catch(isBodylessHandler)
}
</script>
<template>
    <section v-if="props.topic" class="flex justify-between">
        <div class=" flex-grow">
            <section v-if="!props.topic.StringOnly" class="flex justify-between items-center">
                <p>Hash: {{ props.topic.get_hash()! }}</p>
                <button class="aspect-square w-fit" @click="() => writeText(props.topic.get_hash()!.valueOf())"><span
                        class="material-icons icon-center">content_copy</span></button>
            </section>
            <section v-if="!props.topic.HashOnly" class="flex justify-between items-center">
                <p class="">String repr: {{ props.topic.get_string()! }}</p>
                <button class="aspect-square w-fit" @click="writeText(props.topic.get_string()!.valueOf())"><span
                        class="material-icons icon-center">content_copy</span></button>
            </section>
        </div>
        <ToolTip v-if="props.unsub" help-text="unsubscribe"> <button class="w-8 py-2 px-1 ml-1" @click="unsubscribe">
                <span class="material-icons text-2xl text-center align-baseline">speaker_notes_offf</span>
            </button></ToolTip>

    </section>
    <section v-else>
        <p>Loading topic info...</p>
    </section>
</template>