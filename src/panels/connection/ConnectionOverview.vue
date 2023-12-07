<script setup>
import {ref} from "vue"
import {invoke} from "@tauri-apps/api/tauri"
import {listen} from "@tauri-apps/api/event"
let peer_to_dial = ref("")
let dial_events = ref([])
function dial(){
    let listen_handle = listen("dial-event-" + peer_to_dial.value,(ev)=>{console.log(ev)})
    invoke("plugin:swarm|dial",{address:peer_to_dial.value}).then((v)=>{console.log("Dialed. ",v);listen_handle()})
}
</script>

<template>
    <div>
        <p>Dial a peer</p>
        <input v-model="peer_to_dial"/>
        <button @click="dial">Dial</button>
    </div>
</template>