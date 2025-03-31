<script setup lang="ts">
import { ref, Ref } from 'vue';
import { TopicRecord, Topic, TopicInfo } from './types';
import { invoke } from '@tauri-apps/api/core';
import { isBodylessHandler } from '../../../utils';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import ToolTip from '../../../components/ToolTip.vue';

const props = defineProps({
    topic: {
        type: TopicRecord,
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
    invoke("plugin:owlnest-gossipsub|unsubscribe", { topic: Topic.hash_only(props.topic.get_hash()) })
        .then((_v) => emit('unsubscribe'))
        .catch(isBodylessHandler)
}
</script>
<template>
    <section v-if="props.topic">
        <div class="flex">
            <div style="width: calc(100% - 1.5rem);" class="p-1">
                <section class="flex justify-between items-center">
                    <p class="text-autowrap">Hash: {{ props.topic.get_hash().hash }}</p>
                    <button class="aspect-square w-fit"
                        @click="() => writeText(props.topic.get_hash().hash)"><span
                            class="material-icons icon-center">content_copy</span></button>
                </section>
                <section v-if="!props.topic.HashOnly" class="flex justify-between items-center">
                    <p class="text-autowrap">String repr: {{ props.topic.get_string()! }}</p>
                    <button class="aspect-square w-fit" @click="writeText(props.topic.get_string()!.valueOf())"><span
                            class="material-icons icon-center">content_copy</span></button>
                </section>
            </div>
            <ToolTip v-if="props.unsub" help-text="unsubscribe">
                <button class="w-8 py-3 px-1 ml-1" @click="unsubscribe">
                    <span class="material-icons text-2xl text-center align-baseline">speaker_notes_offf</span>
                </button>
            </ToolTip>
        </div>
    </section>
    <section v-else>
        <p>Loading topic info...</p>
    </section>
</template>