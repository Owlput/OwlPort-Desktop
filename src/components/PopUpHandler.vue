<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from "vue";
import { listen, emit } from "@tauri-apps/api/event";
import DefaultPopup from "../popups/DefaultPopup.vue";
import { isBodylessHandler } from "../utils";

defineOptions({
  components: { DefaultPopup },
});

let popups = ref([]);
let current_popup_pointer = ref(0);
function previous_popup() {
  if (current_popup_pointer.value > 0) {
    current_popup_pointer.value -= 1;
  }
}
function next_popup() {
  if (current_popup_pointer.value + 1 < popups.value.length) {
    current_popup_pointer.value += 1;
  }
}
function add_popup(new_popup) {
  popups.value.push(new_popup);
  if (new_popup.timeout > 0) {
    setTimeout(() => removePopup(new_popup.stamp), new_popup.timeout);
  }
  console.log(popups.value);
}
function removePopup(stamp) {
  let pos = popups.value.findIndex((item) => item.stamp == stamp);
  if (current_popup_pointer.value >= pos) {
    current_popup_pointer.value -= 1;
  }
  popups.value.splice(pos, 1);
}
listen("newPopup", (popup) => {
  /*
    A valid popup should look like this:
    {
      timeout: number,   // In seconds. If the number is 0, it never times out.
      stamp: String,     // Unique identifier for the popup.
      component: String, // The path for custom popup.
      props: String,     // Props for the component.
    }

  */
  add_popup(popup.payload);
}).catch(isBodylessHandler);
</script>
<template>
  <div
    v-if="popups[current_popup_pointer]"
    class="w-fit h-fit fixed right-0 bottom-0 mr-4 mb-4 flex flex-col"
  >
    <div
      v-if="popups.length > 1"
      class="flex flex-row items-center self-end mb-2"
    >
      <button
        class="popBtn"
        v-show="current_popup_pointer + 1 > 1"
        @click="previous_popup"
      >
        <span class="material-icons m-auto pop-select">arrow_back</span>
      </button>
      <p class="md:text-2xl my-auto mx-1">{{ current_popup_pointer + 1 }}</p>
      <button
        class="popBtn"
        v-show="current_popup_pointer + 1 < popups.length"
        @click="next_popup"
      >
        <span class="material-icons m-auto pop-select">arrow_forward</span>
      </button>
    </div>
    <KeepAlive>
      <component
        :is="popups[current_popup_pointer].component"
        v-bind="{
          ...JSON.parse(popups[current_popup_pointer].props),
          close: () => {
            removePopup(popups[current_popup_pointer].stamp);
          },
        }"
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
