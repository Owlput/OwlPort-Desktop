<script setup>
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';

const peer_to_query = ref("");
const events = ref([])

function query(){
    invoke("plugin:owlnest-kad|lookup",{peerId:peer_to_query.value}).then((result)=>events.value.push(ev));
}

</script>

<template>
    <section class="single-input px-4">
        <input v-model="peer_to_query" @keydown.enter.exact.prevent="query"/>
        <button @click="query">Query</button>
    </section>
    <ul>
        <li v-for="ev in events">
        <p>{{ JSON.stringify(ev) }}</p>
        </li>
    </ul>
</template>