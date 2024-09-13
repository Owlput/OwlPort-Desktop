<script setup lang="ts">
const props = defineProps({
    path: {
        type: String,
        required: true
    }
})
</script>
<template>
    <li @click="() => $router.push(props.path.valueOf())"
        :class="$route.path.slice(0, props.path.length) == props.path ? 'tab tab-selected' : 'tab'"
        style="height: 100%">

        <slot></slot>
    </li>
</template>
<style>
.tab:hover{
    background-color: #eee;
}

.tab-container *.tab {
    border-bottom: #ddd solid 0.125rem;
    padding-top: 0.5rem;
}
.tab-container *.tab-selected {
    border-bottom: #2a7fff solid 0.125rem !important;
}
</style>