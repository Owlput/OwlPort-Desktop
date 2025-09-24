<script setup lang="ts">
import PopUpHandler from "./components/PopUpHandler.vue";
import SideBar from "./components/SideBar.vue";
import { useRoute, useRouter } from "vue-router";
import { isBodyless } from "./utils";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
const router = useRouter();
document.addEventListener("keyup", (ev: KeyboardEvent) => {
  if (!ev.isTrusted || ev.isComposing) {
    return;
  }
  if (ev.key == "Escape") {
    router.back();
  }
});

let appWindowLabel;
if (!isBodyless()) { console.log("backend connected"); appWindowLabel = getCurrentWebviewWindow().label; } else {
  let route = useRoute();
  appWindowLabel = route.fullPath.split("/")[1]
}
</script>
<template>
  <v-app>
    <div id="main-grid">
      <section class="w-[5rem]" v-if="$route.path.slice(0, 3) != '/app'">
        <SideBar></SideBar>
      </section>
      <div>
        <RouterView v-slot="{ Component }">
            <component :is="Component" />
        </RouterView>
      </div>
    </div>
    <PopUpHandler></PopUpHandler>
  </v-app>
</template>

<style>
#main-grid {
  display: grid;
  grid-template-columns: 5rem auto;
  height: 100vh;
  width: 100vw;
}

#main-grid>div {
  width: calc(100vw - 5rem);
  height: 100vh;
}
</style>
