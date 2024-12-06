<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import { Popup } from '../../components/Types';
</script>

<template>
    <p>Developer Options</p>
    <button
        @click="() => invoke('plugin:libp2p-gossipsub|try_map_string_to_hash', { topic: 'TestTopic' }).then(v => console.log(v))">Test
        Map String to TopicHash</button>
    <button @click="() => emit('newPopup', new Popup(Date.now(), 'DefaultPopup', { message: 'what' }))">Test popup from
        frontend</button>
    <button @click="() => invoke('plugin:owlnest-developer-options|swarm_event_listener')">Swarm Event Listener</button>
</template>