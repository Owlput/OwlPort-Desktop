<script setup>
import {ref} from "vue"
import {invoke} from "@tauri-apps/api/tauri"
let local_peer_id = ref("")
invoke("plugin:swarm|get_local_peer_id").then((res)=>{ local_peer_id.value = res })

</script>

<template>
    <p class="text-center">This is Home</p>
    <button @click="()=>{
        invoke('plugin:popup-tester|emit_test_event')
    }">Test popup</button>
    <p>Local peer ID: {{ local_peer_id }}</p>
</template> 