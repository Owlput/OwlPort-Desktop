<script setup>
const props = defineProps({
  reason: {
    type: String,
    required: true,
  },
  dismiss: {
    type: Function,
    required: true,
  },
  retry: {
    type: Function,
    required: true,
  },
});
</script>

<template>
  <div class="bg-[#F2C037] rounded-md p-2">
    <p>Heartbeat to Nest failed with</p>
    <p class="text-center">{{ reason }}</p>
    <div class="flex justify-evenly m-2">
      <button @click="props.dismiss"><p>Dismiss</p></button>
      <button @click="props.retry"><p>Retry</p></button>
    </div>
  </div>
</template>

<style scoped>
p {
  margin: 0.5rem;
}

</style>
