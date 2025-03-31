<script setup lang="ts">
import { ref } from 'vue';

const isExposed = ref(false);
</script>
<template>
    <div class="flex w-full h-full">
        <div id="slot-wrapper"
        class=" hover:cursor-pointer"
        @click="() => isExposed = !isExposed">
            <slot name="main"></slot>
        </div>
        <div id="more-wrapper" v-if="isExposed">
            <slot name="more"></slot>
        </div>
    </div>
</template>
<style scoped>
#slot-wrapper {
    margin-right: 0rem;
    width: 100%;
    height: 100%;
}

#more-wrapper {
    right: 0px;
}
</style>