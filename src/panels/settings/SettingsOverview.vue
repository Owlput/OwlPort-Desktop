<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
</script>

<template>
    <p>Developer Options</p>
    <button @click="()=>invoke('plugin:libp2p-gossipsub|try_map_string_to_hash',{topic:'TestTopic'}).then(v=>console.log(v))">Test Map String to TopicHash</button>
</template>