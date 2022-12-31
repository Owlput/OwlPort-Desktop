<script setup>
import { onMounted, onUnmounted, ref, computed } from "vue";

const popups = ref(new Map());
let current = ref(0);
const currentPopup = computed(() => {
  let pos = 0;
  if (popups.value.size == 0) return null;
  for (let [key, val] of popups.value) {
    if (pos == current.value) return val;
    pos += 1;
  }
});
const createPopup = (ev) => {
  /* 
  A valid popup should have the following structure:
  {
    handlerType:"type",
    stamp:`something unique`,
    timeout:`when to remove the popup in miliseconds`,
    popup:{
      component:`component for the popup`,
      props:`props for the component`,
    },
  }
*/
  if (!window.sessionStorage.getItem(ev.detail.handlerType)) {
    if (ev.detail.timeout) {
      setTimeout(() => {
        deletePopup(ev);
      }, ev.detail.timeout);
    }
    popups.value.set(ev.detail.stamp, ev.detail.popup);
  }
};
const deletePopup = (ev) => {
  if (current.value + 1 == popups.value.size) current.value -= 1;
  popups.value.delete(ev.detail.stamp);
};
const disablePopup = (ev) => {
  window.sessionStorage.setItem(ev.detail.handlerType, "disabled");
};
const enablePopup = (ev) => {
  window.sessionStorage.removeItem(ev.detail.handlerType);
};

onMounted(() => {
  addEventListener("createPopup", createPopup);
  addEventListener("deletePopup", deletePopup);
  addEventListener("disablePopup", disablePopup);
  addEventListener("enablePopup", enablePopup);
});
onUnmounted(() => {
  removeEventListener("createPopup", createPopup);
  removeEventListener("deletePopup", deletePopup);
  removeEventListener("disablePopup", disablePopup);
  removeEventListener("enablePopup", enablePopup);
});
</script>
<template>
  <div
    v-if="currentPopup?.props"
    class="w-fit h-fit fixed right-0 bottom-0 mr-4 mb-4 flex flex-col"
  >
    <div
      v-if="popups.size > 1"
      class="flex flex-row items-center self-end mb-2"
    >
      <button
        class="popBtn"
        v-show="current + 1 != 1"
        @click="
          () => {
            if (current > 0) current -= 1;
          }
        "
      >
        <span class="material-icons m-auto pop-select">arrow_back</span>
      </button>
      <p class="md:text-2xl my-auto mx-1">{{ current + 1 }}</p>
      <button
        class="popBtn"
        v-show="current + 1 != popups.size"
        @click="
          () => {
            if (current + 1 < popups.size) current += 1;
          }
        "
      >
        <span class="material-icons m-auto pop-select">arrow_forward</span>
      </button>
    </div>
    <KeepAlive>
      <component
        :is="currentPopup.component"
        v-bind="currentPopup.props"
      ></component>
    </KeepAlive>
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
