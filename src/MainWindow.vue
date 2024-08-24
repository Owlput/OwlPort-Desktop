<script setup>
import PopUpHandler from "./components/PopUpHandler.vue";
import SideBar from "./components/SideBar.vue";
import { useRouter } from "vue-router";
const router = useRouter();
document.addEventListener("keyup",(ev)=>{
  if ( !ev.isTrusted || ev.isComposing){
    return;
  }
  if (ev.key == "Escape"){
    router.back()
  }
})

</script>
<template>
  <div id="main-grid">
    <section class="w-[5rem]" v-if="$route.path.split(0, 3) != '/app'">
      <SideBar></SideBar>
    </section>
    <div>
      <RouterView v-slot="{ Component }">
        <KeepAlive exclude="Overview">
          <component :is="Component" />
        </KeepAlive>
      </RouterView>
    </div>
  </div>
  <PopUpHandler></PopUpHandler>
</template>

<style>
#main-grid {
  display: grid;
  grid-template-columns: 5rem auto;
  height: 100vh;
  width: 100vw;
}
#main-grid > div {
  width: calc(100vw - 5rem);
  height: 100vh;
}
</style>
