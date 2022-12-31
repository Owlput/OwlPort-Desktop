<script setup>
const props = defineProps({
  msg: {
    type: String,
    required: true,
  },
});
function removeOverlay() {
  dispatchEvent(new CustomEvent("removeOverlay"));
}
</script>
<template>
  <div class="overlayWrapper flex flex-row items-center bg-[rgba(0,0,0,0.2)]" @click="removeOverlay">
    <div class="w-[10rem] h-[10rem] bg-slate-500">
      <p>Test Overlay:{{ props.msg }}</p>
    </div>
  </div>
</template>
