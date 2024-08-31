<script setup lang="ts">
import { ref, Ref } from "vue";
import { TopicMessage } from "./types";

const message_history: Ref<Array<TopicMessage>> = ref([]);
</script>
<template>
  <ul>
    <li v-for="msg in message_history" :style="msg.verified?{backgroundColor: 'green'}:{backgroundColor: 'red'}">
        <div>From: <p :style="msg.trusted?{color:'gold'}:{}">{{ msg.source_peer }}</p></div>
        <p>{{ msg.message }}</p>
    </li>
  </ul>
</template>
