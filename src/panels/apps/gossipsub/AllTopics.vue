<script setup lang="ts">
import { ref, Ref } from "vue";
import SearchBar from "../../../components/SearchBar.vue"
import {Topic} from "./types"

const topics:Ref<Array<Topic>> = ref([]);
const search_text = ref("");

</script>

<template>
    <SearchBar v-model="search_text" :refresh="()=>{}" :place-holder="'Search for a topic'"></SearchBar>
  <ul>
    <li v-for="topic in topics">
        <p>Topic hash: {{ topic.hash}}</p>
    </li>
  </ul>
</template>
