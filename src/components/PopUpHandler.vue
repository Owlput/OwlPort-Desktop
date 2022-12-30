<script setup>
import { onMounted, onUnmounted, ref, shallowRef } from "vue";
import FailedHeartbeat from "./Popups/FailedHeartbeat.vue";

const popups = ref([
  { component: shallowRef(FailedHeartbeat), props: { reason: "some_reason" } },
  {
    component: shallowRef(FailedHeartbeat),
    props: { reason: "another_reason" },
  },
]);
let currentPopups = ref(0);
const createPopup = (ev) => {
  if (!window.sessionStorage.getItem(ev.detail.typeHandler)) {
    if (ev.detail.timeout)
      setTimeout(ev.detail.timeout, () => {
        if (currentPopups.value + 1 == popups.value.length) currentPopups -= 1;
        popups.value.splice(popups.value.length - 1, 1);
      });
    // popups.value.push({

    // })
  }
};
const disablePopup = (ev) => {
  window.sessionStorage.setItem(ev.detail.typeHandler, "disabled");
};
const enablePopup = (ev) => {
  window.sessionStorage.removeItem(ev.detail.typeHandler);
};

onMounted(() => {
  addEventListener("createPopup", () => {});
  addEventListener("disablePopup", disablePopup);
  addEventListener("enablePopup", enablePopup);
});
onUnmounted(() => {
  removeEventListener("createPopup", createPopup);
  removeEventListener("disablePopup", disablePopup);
  removeEventListener("enablePopup", enablePopup);
});
</script>
<template>
  <div
    v-if="popups.length != 0"
    class="w-fit h-fit fixed right-0 bottom-0 mr-4 mb-4"
  >
    <div
      v-if="popups.length > 1"
      class="flex flex-row items-center absolute"
      style="top: -2rem; right: 0px"
    >
      <button
        class="w-2 h-2"
        @click="
          () => {
            if (currentPopups > 0) currentPopups -= 1;
          }
        "
      >
        <p class="text-center mb-0">&lt;</p>
      </button>
      <p class="m-0">{{ currentPopups + 1 }}</p>
      <button
        class="w-4 h-4"
        @click="
          () => {
            if (currentPopups + 1 < popups.length) currentPopups += 1;
          }
        "
      >
      <p>
        &gt;</p>
      </button>
    </div>
    <KeepAlive>
      <component
        :is="popups[currentPopups].component"
        v-bind="popups[currentPopups].props"
      ></component>
    </KeepAlive>
  </div>
</template>
