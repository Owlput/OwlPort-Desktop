<script setup>
const searchText = defineModel()
const props = defineProps({
    refresh:Function,
    placeHolder: String
})
</script>
<template>
    <input
      class="h-8 w-[80%] px-4"
      :placeholder="props.placeHolder"
      v-model.lazy="searchText"
    />
    <button class="h-8 w-8" @click="props.refresh">
      <span class="material-icons h-full w-full">refresh</span>
    </button>
</template>
