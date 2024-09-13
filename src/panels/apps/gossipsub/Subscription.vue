<script setup lang="ts">
import { computed, ComputedRef, Ref, ref, shallowRef, ShallowRef } from 'vue';
import { ReadableTopic } from './types';
import SearchBar from '../../../components/SearchBar.vue';
import { invoke } from '@tauri-apps/api';
import { isBodylessHandler } from '../../../utils';
import Topic from "./Topic.vue"
import ToolTip from '../../../components/ToolTip.vue';

const search_text: Ref<String> = ref("");
const topic_to_subscribe: Ref<String> = ref("")
const subscribed_topics: ShallowRef<Array<ReadableTopic>> = shallowRef([]);
const filtered_topics: ComputedRef<Array<ReadableTopic>> = computed(() => {
    if (search_text.value.length === 0) return subscribed_topics.value;
    if (search_text.value.startsWith('#'))
        return subscribed_topics.value.filter((v) => v.get_hash()?.includes(search_text.value.slice(1)));
    return subscribed_topics.value.filter((v) => v.get_string()?.includes(search_text.value.valueOf()));
});
function subscribe() {
    if (topic_to_subscribe.value.length === 0 || topic_to_subscribe.value[0] === '#') return;
    invoke<boolean>("plugin:libp2p-gossipsub|subscribe", { topic: { StringOnly: topic_to_subscribe.value } })
        .then((_v) => {
            topic_to_subscribe.value = ""; refresh()
        }).catch(isBodylessHandler);
}
function refresh() {
    invoke<Array<ReadableTopic>>("plugin:libp2p-gossipsub|subscribed_topics").then((v) => {
        subscribed_topics.value = v.map(v => new ReadableTopic(v));
    }).catch(isBodylessHandler)
}
refresh()
</script>
<template>
    <div class="overflow-hidden">
        <section>
            <SearchBar v-model="search_text" :refresh="refresh"
                place-holder="Search for a subscribed topic or hash(start with #)." />
        </section>
        <ul class="topic-list">
            <li v-for="topic in filtered_topics" class="h-fit w-full">
                <Topic :topic="topic" @unsubscribe="refresh" unsub></Topic>
            </li>
        </ul>
        <section class="w-full p-2">
            <input class="h-12 p-2" style="width: calc(100% - 3.5rem);" @keyup.enter="subscribe"
                placeholder="Subscribe to a topic, topic hash is not allowed." v-model.lazy="topic_to_subscribe" />
            <ToolTip help-text="subscribe" top class="float-right">
                <button class="aspect-square h-12" @click="subscribe">
                    <img class="m-auto" src="../../../assets/chat_add_on_E8EAED_F0_w400.svg" />
                </button>
            </ToolTip>

        </section>
    </div>
</template>
<style>
.topic-list {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    overflow: auto;
    width: 100vw;
    gap: 4px;
    padding: 1rem;
    height: calc(100vh - 10.5rem);
}

.topic-list>li {
    border: 1px solid gainsboro;
    padding: 0.5rem 1rem;
    width: 100%;
}
</style>