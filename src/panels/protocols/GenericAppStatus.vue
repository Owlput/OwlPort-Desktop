<script setup>
import {invoke} from '@tauri-apps/api/tauri'
const props = defineProps({
    appName: String,
    windowEntrypoint: String
});
</script>

<template>
    <section class="flex flex-row justify-between items-center border px-4 py-2">
        <p class="hover:cursor-pointer " @click="() => invoke(props.windowEntrypoint)">
            {{ props.appName }}
        </p>
        <span class="material-icons">
            chevron_right
        </span>
    </section>
</template>