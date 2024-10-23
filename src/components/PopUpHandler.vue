<script setup lang="ts">
import { defineAsyncComponent, ref, Ref, watchEffect } from "vue";
import { listen, } from "@tauri-apps/api/event";
import { isBodylessHandler } from "../utils";
import { Popup } from "./Types";

defineOptions({
  components: {
    "DefaultPopup": defineAsyncComponent(() => import("../popups/DefaultPopup.vue"))
  }
})

let popups: Ref<Array<Popup>> = ref([]);
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
function add_popup(new_popup: Popup) {
  popups.value.push(new_popup);
  if (new_popup.timeout > 0) {
    setTimeout(() => removePopup(new_popup.timestamp), new_popup.timeout);
  }
}
function removePopup(timestamp: number) {
  let pos = popups.value.findIndex((item) => item.timestamp == timestamp);
  if (current_popup_pointer.value >= pos && current_popup_pointer.value > 0) {
    current_popup_pointer.value -= 1;
  }
  popups.value.splice(pos, 1);
}
listen<Popup>("newPopup", (popup) => {
  /*
    A valid popup should look like this:
    {
      timeout: number,   // In seconds. If the number is 0, it never times out.
      timestamp: String,     // Unique identifier for the popup.
      component_path: String, // The path for custom popup.
      component_props: String,     // Props for the component.
    }

  */
  console.log("received new popup:",popup.payload)
  add_popup(popup.payload);
}).catch(isBodylessHandler);
</script>
<template>
  <div v-if="popups[current_popup_pointer]" class="w-fit h-fit fixed right-0 bottom-0 mr-4 mb-4 flex flex-col">
    <div v-if="popups.length > 1" class="flex flex-row items-center self-end mb-2">
      <button class="popBtn" v-show="current_popup_pointer + 1 > 1" @click="previous_popup">
        <span class="material-icons m-auto pop-select">arrow_back</span>
      </button>
      <p class="md:text-2xl my-auto mx-1">{{ current_popup_pointer + 1 }}</p>
      <button class="popBtn" v-show="current_popup_pointer + 1 < popups.length" @click="next_popup">
        <span class="material-icons m-auto pop-select">arrow_forward</span>
      </button>
    </div>
    <KeepAlive>
      <component :is="popups[current_popup_pointer].component_path" v-bind="{
        ...popups[current_popup_pointer].component_props,
        close: () => {
          removePopup(popups[current_popup_pointer].timestamp);
        },
      }"></component>
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
