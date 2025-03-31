<script setup lang="ts">
import { computed, ComputedRef, onMounted, Ref, ref, shallowRef, ShallowRef } from 'vue';
import { TopicHash, TopicRecord } from './types';
import SearchBar from '../../../components/SearchBar.vue';
import { invoke } from '@tauri-apps/api/core';
import { isBodylessHandler } from '../../../utils';
import Topic from "./Topic.vue";
import { Topic as SubscriptionTopic } from "./types";
import ToolTip from '../../../components/ToolTip.vue';

const search_text: Ref<string> = ref("");
const topic_to_subscribe: Ref<string> = ref("");
const hash_type: Ref<string> = ref("Identity");
const subscribed_topics: ShallowRef<Array<TopicRecord>> = shallowRef([]);
const filtered_topics: ComputedRef<Array<TopicRecord>> = computed(() => {
    if (search_text.value.length === 0) return subscribed_topics.value;
    if (search_text.value.startsWith('#'))
        return subscribed_topics.value.filter((v) => v.get_hash()?.hash.includes(search_text.value.slice(1)));
    return subscribed_topics.value.filter((v) => v.get_string()?.includes(search_text.value.valueOf()));
});
function subscribe() {
    if (topic_to_subscribe.value.length === 0) return;
    if (topic_to_subscribe.value.startsWith('#')) {
        invoke<boolean>("plugin:owlnest-gossipsub|subscribe", { topic: SubscriptionTopic.hash_only(new TopicHash(topic_to_subscribe.value.slice(1))) })
            .then((_v) => {
                topic_to_subscribe.value = ""; refresh()
            }).catch(isBodylessHandler);
        return
    }
    switch (hash_type.value) {
        case "Identity": {
            invoke<boolean>(
                "plugin:owlnest-gossipsub|subscribe",
                { topic: SubscriptionTopic.identity_string(topic_to_subscribe.value) })
                .then((_v) => { topic_to_subscribe.value = ""; refresh() })
                .catch(isBodylessHandler);
            break;
        }
        case "Sha256": {
            invoke<boolean>(
                "plugin:owlnest-gossipsub|subscribe",
                { topic: SubscriptionTopic.sha256_string(topic_to_subscribe.value) })
                .then((_v) => { topic_to_subscribe.value = ""; refresh() })
                .catch(isBodylessHandler);
            break;
        }
        default: return;
    }
}
function refresh() {
    invoke<Array<TopicRecord>>("plugin:owlnest-gossipsub|subscribed_topics").then((v) => {
        subscribed_topics.value = v.map(v => new TopicRecord(v));
    }).catch(isBodylessHandler)
}
onMounted(()=>refresh())
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
        <section class="flex w-full p-2 absolute bottom-0">
            <input class="h-12 p-2" style="width: calc(100% - 9rem);" @keyup.enter="subscribe"
                placeholder="Subscribe to a topic. Start with '#' for a topic hash."
                v-model.lazy="topic_to_subscribe" />
            <ToolTip help-text="select the hasher for topic string" top>
                <select v-model="hash_type" class="h-12 mx-1 border rounded-md shadow-sm">
                    <option value="Identity">Identity</option>
                    <option value="Sha256">SHA256</option>
                </select>
            </ToolTip>
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