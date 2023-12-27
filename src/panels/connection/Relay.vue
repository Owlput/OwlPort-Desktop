<script setup>
import {ref} from 'vue'

const relay_list = ref([])

</script>
<template>
    <p>Connected relays:</p>
    <ul>
        <li v-for="relay in relay_list"></li>
    </ul>
</template>