<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { isBodyless } from "../../utils";
import { useRouter } from "vue-router";
import { onMounted, onUnmounted, Ref, ref } from "vue";

const props = defineProps({
  appName: String,
  pluginName: {
    type: String,
    required: true,
  },
  alternativePath: String,
  showNumPeers: {
    type: Boolean,
    default: false
  },
});
const router = useRouter();
const num_peers: Ref<number | undefined> = ref(undefined);

const refresh_interval_id: Ref<number | null> = ref(null)
onMounted(() => {
  if (props.showNumPeers) {
    refresh_interval_id.value = setInterval(() => {
      invoke<Array<string>>(props.pluginName + "|list_connected").catch((e) => { console.log(e) }).then((list) => num_peers.value = list?.length)
    }, 1000)
  }
})

onUnmounted(() => {
  clearInterval(refresh_interval_id.value!)
})

function spawn_or_focus() {
  let is_bodyless = isBodyless();
  if (is_bodyless) {
    if (props.alternativePath) return router.push(props.alternativePath);
    else
      return console.error(
        "Alternative path not configured when navigating in bodyless mode."
      );
  }
  invoke(props.pluginName + "|spawn_window").catch((e: Error) => console.error(e.message));
}


</script>

<template>
  <v-card class="px-4 py-2 hover:cursor-pointer" @click="spawn_or_focus">
    <section class="m-2 app-three-seg">
      <p class="hover:cursor-pointer">
        {{ props.appName }}
      </p>
      <p v-if="num_peers">Peers: {{ num_peers }}</p>
      <span class="material-icons float-right"> chevron_right </span>
    </section>
  </v-card>
</template>
<style>
.app-three-seg {
  display: grid;
  grid-template-columns: 1fr auto max-content;
  align-items: center;
}
</style>