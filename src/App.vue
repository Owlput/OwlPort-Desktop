<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";
import SideNav from "./components/SideNav.vue";
import PopUpHandler from "./components/PopUpHandler.vue";
import Loopings from "./loopings/Loopings.vue";
import { computed } from "@vue/reactivity";

const splitterPercent = ref(70);
const navShowDesp = computed(()=>{
  return splitterPercent.value > 80
})
</script>

<template>
  <QSplitter v-model="splitterPercent" unit="px" :limits="[60,100]" class="h-screen" emitImmediately>
    <template v-slot:before> <SideNav :showDesp="navShowDesp"></SideNav></template>
    <template v-slot:after> <RouterView></RouterView></template>
  </QSplitter>
  <Loopings></Loopings>
  <PopUpHandler></PopUpHandler>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
