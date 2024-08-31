<script setup lang="ts">
const searchText = defineModel();
const props = defineProps<{
  refresh: Function;
  placeHolder: String;
}>();
</script>
<template>
  <section class="w-full">
    <input
      class="h-8 px-4 w-max m-2"
      :placeholder="props.placeHolder.valueOf()"
      v-model.lazy="searchText"
    />
    <button class="text-center aspect-square h-12 m-2" @click="props.refresh">
      <span class="material-icons text-3xl">refresh</span>
    </button>
  </section>
</template>
