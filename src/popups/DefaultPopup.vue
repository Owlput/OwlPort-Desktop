<script setup>
const props = defineProps({
  title: String,
  message: String,
  close:Function,
});
</script>

<template>
  <div>
    <h2>{{ props.title ? props.title : "" }}</h2>
    <p>{{ props.message }}</p>
  </div>
  <button @click="close">Close</button>
</template>
