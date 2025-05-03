<script setup lang="ts">
import { onMounted, Ref, ref } from 'vue';
import { TopicHash, TopicRecord } from './types';
import { invoke } from '@tauri-apps/api/core';
import { isBodylessHandler } from '../../../utils';
import Topic from "./Topic.vue";
import { Topic as SubscriptionTopic } from "./types";
import SearchDisplay from '../../../components/SearchDisplay.vue';

const topic_to_subscribe: Ref<string> = ref("");
const hash_type: Ref<string> = ref("identity");
const hash_type_items = [{ name: 'Identity', value: "identity" }, { name: 'SHA256', value: "sha256" }];
function topic_filter(source: TopicRecord, search_text: string): boolean {
    if (search_text.startsWith('#'))
        return source.get_hash()?.includes(search_text.slice(1)) as boolean
    return source.get_string()?.includes(search_text) as boolean
};
function subscribe() {
    console.log(`subscribe called, ${topic_to_subscribe.value}`)
    if (topic_to_subscribe.value.length === 0) return;
    let param;
    if (topic_to_subscribe.value.startsWith('#')) {
        param = { topic: SubscriptionTopic.hash_only(new TopicHash(topic_to_subscribe.value.slice(1))) }
        invoke<boolean>("plugin:owlnest-gossipsub|subscribe", param)
            .then((_v) => {
                topic_to_subscribe.value = ""; refresh()
            }).catch(isBodylessHandler);
        return;
    }
    switch (hash_type.value) {
        case "identity": {
            param = { topic: SubscriptionTopic.identity_string(topic_to_subscribe.value) }
            break;
        }
        case "sha256": {
            param = { topic: SubscriptionTopic.sha256_string(topic_to_subscribe.value) }
            break;
        }
        default: return;
    }
    invoke<boolean>("plugin:owlnest-gossipsub|subscribe", param)
        .then((_v) => {
            topic_to_subscribe.value = ""; refresh()
        }).catch(isBodylessHandler);
}
function refresh(): Promise<Array<TopicRecord>> {
    return invoke<Array<TopicRecord>>("plugin:owlnest-gossipsub|subscribed_topics").then((v) => {
        console.log(v)
        return v.map(v => new TopicRecord(v))
    }
    )
}
onMounted(() => refresh())
</script>
<template>
    <div class="p-4">
        <SearchDisplay :criteria="topic_filter" :get-or-refresh="refresh" v-slot="slotProps"
            place-holder="Search for topic or topic hash(start with #)" :min-item-size="108"
            scroller-height-expr="100vh - 13.5rem">
            <div class="py-1">
                <div class="border border-gray-200 rounded-sm shadow-sm">
                    <Topic :topic="slotProps.item" @unsubscribe="refresh" unsub></Topic>
                </div>
            </div>
        </SearchDisplay>
    </div>
    <form class="flex px-4" @submit.prevent="subscribe">
        <v-text-field style="width: calc(100% - 12rem);" hide-details
            placeholder="Subscribe to a topic. Start with '#' for a topic hash." v-model.lazy="topic_to_subscribe" />
        <v-select class="h-12 mx-1 border rounded-md shadow-sm" label="Hasher" :items="hash_type_items" hide-details
            v-model="hash_type" item-title="name" item-value="value">
        </v-select>
        <v-btn class="aspect-square h-12 cursor-pointer" type="submit" height="3.5rem">
            <span class="mdi-forum-plus-outline mdi text-center text-2xl"></span>
            <v-tooltip activator="parent" location="top" open-on-hover open-delay="2000">
                Subscribe
            </v-tooltip>
        </v-btn>
    </form>
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