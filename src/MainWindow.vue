<script setup>
import PopUpHandler from "./components/PopUpHandler.vue";
import SideBar from "./components/SideBar.vue";
</script>
<template>
  <div id="main-grid">
    <section class="w-[5rem]" v-if="$route.path.split(0, 3) != '/app'">
      <SideBar></SideBar>
    </section>
    <main>
      <RouterView v-slot="{ Component }">
        <KeepAlive exclude="Overview">
          <component :is="Component" />
        </KeepAlive>
      </RouterView>
    </main>
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
#main-grid > main {
  width: calc(100vw - 5rem);
  height: 100vh;
}
</style>
