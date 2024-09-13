<script setup lang="ts">
const searchText = defineModel();
const emit = defineEmits(["searchSubmit"])
const props = defineProps(
  {
    refresh: Function,
    placeHolder: String,
  }
);
</script>
<template>
  <section class="w-full p-2">
    <input class="h-12 p-2" style="width: calc(100% - 3.5rem);" :placeholder="props.placeHolder?.valueOf()"
      v-model.lazy="searchText" @submit="emit('searchSubmit')" />
    <button v-if="typeof props.refresh !== undefined" class="aspect-square h-12 float-right"
      @click="() => props.refresh!()">
      <span class="material-icons text-3xl icon-center">refresh</span>
    </button>
  </section>
</template>
