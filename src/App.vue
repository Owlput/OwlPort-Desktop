<script setup>
import SideBar from "./components/SideBar.vue";
import PopUpHandler from "./components/PopUpHandler.vue";
</script>

<template>
  <div id="layout-grid">
    <section><SideBar></SideBar></section>
    <main class="w-full">
      <RouterView v-slot="{ Component }">
        <KeepAlive>
          <component :is="Component" />
        </KeepAlive>
      </RouterView>
    </main>
  </div>
  <PopUpHandler></PopUpHandler>
</template>

<style scoped>
#layout-grid {
  display: grid;
  grid-template-columns: minmax(4rem, 5rem) auto;
  height: 100vh;
  width: 100vw;
}
</style>
