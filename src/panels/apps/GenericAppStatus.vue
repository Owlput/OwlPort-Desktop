<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { isBodyless } from "../../utils";
import { useRouter } from "vue-router";

const props = defineProps({
  appName: String,
  windowEntrypoint: {
    type: String,
    required: true,
  },
  alternativePath: String,
});
const router = useRouter();

function spawn_or_redirect() {
  let is_bodyless = isBodyless();
  if (is_bodyless) {
    if (props.alternativePath) return router.push(props.alternativePath);
    else
      return console.error(
        "Alternative path not configured when navigating in bodyless mode."
      );
  }
  invoke(props.windowEntrypoint).catch((e: Error) => console.error(e.message));
}
</script>

<template>
  <section class="flex flex-row justify-between items-center border px-4 py-2 hover:cursor-pointer"
    @click="spawn_or_redirect">
    <p class="hover:cursor-pointer">
      {{ props.appName }}
    </p>
    <span class="material-icons"> chevron_right </span>
  </section>
</template>
