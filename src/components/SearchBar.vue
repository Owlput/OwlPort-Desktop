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
  <form class="flex gap-2 items-start">
    <v-text-field :placeholder="props.placeHolder?.valueOf()"
      type="search" v-model.lazy="searchText"
      @keypress.enter="() => { emit('searchSubmit'); console.log('submitted!') }" />
    <v-btn v-if="typeof props.refresh !== undefined" @click="() => props.refresh!()" height="3.5rem">
      <span class="mdi-refresh mdi text-3xl icon-center"></span>
    </v-btn>
  </form>
</template>
