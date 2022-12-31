<script setup>
import { onMounted, onUnmounted, ref, shallowRef } from "vue";
import TestOverlay from "./overlays/TestOverlay.vue";

const overlays = ref([
  { overlay: { component: shallowRef(TestOverlay), props: { msg: "Hello" } } },
]);
const putOverlay = (ev) => {
  /* 
  A valid overlay should have the following structure:
  {
    overlay:{
      component:`component for the overlay`,
      props:`props for the component`,
    },
  }
*/
  overlays.value.push(ev.detail.overlay);
};
const removeOverlay = () => {
  overlays.value.pop();
};

onMounted(() => {
  addEventListener("putOverlay", putOverlay);
  addEventListener("removeOverlay", removeOverlay);
});
onUnmounted(() => {
  removeEventListener("putOverlay", putOverlay);
  removeEventListener("removeOverlay", removeOverlay);
});
</script>
<template>
  <div class="fixed left-0 top-0 w-screen h-screen" v-if="overlays.length != 0">
    <ol class="relative w-full h-full">
      <li
        v-for="(item, index) in overlays"
        class="relative w-full h-full"
        :style="`z-index:${(index + 1) * 10};`"
      >
        <component
          :is="overlays[index].overlay.component"
          v-bind="overlays[index].overlay.props"
        ></component>
      </li>
    </ol>
  </div>
</template>
<style scoped>
.popBtn {
  min-width: 2rem;
  max-width: 3rem;
  min-height: 2rem;
  max-height: 3rem;
  display: flex;
  flex-direction: row;
  align-items: center;
  border: none;
  box-shadow: none;
  background-color: rgba(255, 255, 255, 0);
}
.pop-select:hover {
  color: #fb8c00;
}
</style>
