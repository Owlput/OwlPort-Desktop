<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import SearchDisplay from '../../../components/SearchDisplay.vue';

function update_list() {
    return invoke<Array<String>>("plugin:owlnest-messaging|list_connected")
}

function filter(source: String, search_text: String): boolean {
    return source.includes(search_text.valueOf())
}

update_list()
</script>

<template>
    <SearchDisplay :criteria="filter" :get-or-refresh="update_list" v-slot="slotProps"
        place-holder="Search for a peer that supports Messaging" :min-item-size="108"
        scroller-height-expr="100vh - 9.5rem">
        <div class="px-4">
            <div class="border border-gray-200 shadow-sm rounded-sm">
                <p class="hover:cursor-pointer" @click="$router.push(`/app/messaging/chat/${slotProps.item}`)">{{
                    slotProps.item }}</p>
            </div>
        </div>
    </SearchDisplay>
</template>