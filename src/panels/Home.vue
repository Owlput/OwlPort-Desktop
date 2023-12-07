<script setup>
import {invoke} from "@tauri-apps/api/tauri"

</script>

<template>
    <p class="text-center">This is Home</p>
    <button @click="()=>{
        invoke('plugin:popup-tester|emit_test_event')
    }">Test popup</button>
</template> 