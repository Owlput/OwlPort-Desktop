<script setup lang="ts">
const props = defineProps({
  title: String,
  message: String,
  close: {
    type: Function,
    required: true
  },
});
</script>

<template>
  <section class=" bg-gray-300">
    <div>
      <h2 v-if="props.title">{{ props.title }}</h2>
      <p>{{ props.message }}</p>
    </div>
    <button @click="() => props.close()">Close</button>
  </section>

</template>
