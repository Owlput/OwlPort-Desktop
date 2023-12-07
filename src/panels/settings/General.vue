<script setup>
import { onMounted, ref } from "vue";
import DisabledPopups from "../../components/DisabledPopups.vue";
import { invoke } from "@tauri-apps/api/tauri";

const showNestConnectionSettings = ref(false);
const generalSettings = ref({});

function applyGeneralSettings() {
  invoke("plugin:configuration|write_config", {
    config: {AppConfig:generalSettings.value},
    path: {AppConfig:null},
  })
    .then(() => console.log("config sccessfully applied"))
    .catch((e) => console.log("failed to apply config:", e));
  document.activeElement.blur();
}
onMounted(()=>{
  invoke("plugin:configuration|read_config",{path:{AppConfig:null}}).then(
    (v)=>generalSettings.value = JSON.parse(v).AppConfig
  ).catch((e)=>console.error(e))
})
</script>
<template>
  <div class="px-16 py-4">
    <section class="m-2">
      <button
        class="w-full"
        @click="
          () => {
            showNestConnectionSettings = !showNestConnectionSettings;
          }
        "
      >
        <p class="float-left m-2">Nest Connection Settings</p>
        <span class="material-icons float-right m-2"
          >expand_{{ showNestConnectionSettings ? "less" : "more" }}</span
        >
      </button>
      <ul v-if="showNestConnectionSettings" class="p-1 bg-slate-300 mx-1">
        <li class="flex flex-row flex-wrap items-center justify-around m-1">
          <p>Nest Address</p>
          <input
            type="text"
            class="w-[50%] min-w-[8rem] px-2"
            @keypress.enter="applyGeneralSettings"
            v-model="generalSettings.nest.address"
          />
        </li>
      </ul>
    </section>
    <DisabledPopups></DisabledPopups>
  </div>
</template>
