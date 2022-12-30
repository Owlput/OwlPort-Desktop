<script setup>
import SideNav from "./components/SideNav.vue";
import PopUpHandler from "./components/PopUpHandler.vue";
import Loopings from "./loopings/Loopings.vue";
</script>

<template>
  <div class="main-grid h-[100vh]">
  <SideNav :showDesp="navShowDesp"></SideNav>
  <div>
  <RouterView></RouterView></div>
</div>
  <Loopings></Loopings>
  <PopUpHandler></PopUpHandler>
</template>
