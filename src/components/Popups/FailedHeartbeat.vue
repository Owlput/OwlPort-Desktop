<script setup>
const props = defineProps({
  reason:{
    type:String,
    required:true,
  },
  dismiss:{
    type:Function,
    required:true,
  },
  retry:{
    type:Function,
    required:true,
  }
});
</script>

<template>
  <div class="bg-[#F2C037] rounded-md">
    <p>Heartbeat to Nest failed with</p>
    <p>{{ reason }}</p>
    <div>
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
