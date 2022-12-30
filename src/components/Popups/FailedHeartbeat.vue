<script setup>
const props = defineProps(["reason"]);
</script>

<template>
  <div class="bg-[#F2C037]">
    <p>Heartbeat failed</p>
    <p>Reason: {{ props.reason }}</p>
  </div>
</template>

<style scoped>
p {
  margin: 0px;
}
</style>
