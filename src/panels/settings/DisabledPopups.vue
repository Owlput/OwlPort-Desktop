<script setup lang="ts">
import { ref, inject, computed } from "vue";

let { disabledPopupsTable } = inject("disabledPopupsTable");
let showDisabledPopups = ref(false);
const disabledPopups = computed(() => {
  return [...disabledPopupsTable.value.keys()];
});

function enablePopup(handlerType) {
  dispatchEvent(new CustomEvent("enablePopup", { detail: { handlerType } }));
}
function seeDisabled() {
  console.log(disabledPopups.value);
}
</script>
<template>
  <section class="m-2">
    <button
      class="w-full"
      @click="
        () => {
          showDisabledPopups = !showDisabledPopups;
        }
      "
    >
      <p class="float-left m-2">Disabled Popups</p>
      <span class="material-icons float-right m-2"
        >expand_{{ showDisabledPopups ? "less" : "more" }}</span
      >
    </button>
    <ul v-if="showDisabledPopups" class="bg-slate-300 shadow-sm p-2 mx-1">
      <li v-if="disabledPopups.length == 0">
        <p class="text-center">No Item Here</p>
      </li>
      <li
        v-for="val in disabledPopups"
        class="flex flex-row justify-around items-center p-2"
      >
        <p>{{ val }}</p>
        <button @click="() => enablePopup(val)">
          <p class="m-1">Enable</p>
        </button>
      </li>
    </ul>
  </section>
</template>
