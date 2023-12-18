<script setup>
import SideBar from "./components/SideBar.vue";
import PopUpHandler from "./components/PopUpHandler.vue";
</script>

<template>
  <div id="layout-grid">
    <section class="w-[5rem]"><SideBar></SideBar></section>
    <main>
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
  grid-template-columns: 5rem auto;
  height: 100vh;
  width: 100vw;
}
main{
  width: calc(100vw - 5rem);
}
</style>
