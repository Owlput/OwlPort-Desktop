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
    <p>{{ reason }}</p>
    <div class="flex justify-evenly m-2">
      <button @click="props.dismiss">Dismiss</button>
      <button @click="props.retry">Retry</button>
    </div>
  </div>
</template>

<style scoped>
p {
  margin: 0px;
}
</style>
