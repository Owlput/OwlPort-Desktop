<script setup>
import { computed } from "vue";
import { useRouter } from "vue-router";

let router = useRouter();
const props = defineProps({
  showDesp: Boolean,
});
const active = computed(() => {
  switch (router.currentRoute.value.path.split("/")[1]) {
    case "":
      return 0;
    case "overview":
      return 1;
    case "connections":
      return 2;
    case "apps":
      return 3;
    case "settings":
      return 4;
  }
});
</script>
<template>
  <div class="w-fit flex flex-col bg-white shadow-md">
    <div class="mx-auto">
      <p>v0.0.0</p>
    </div>
    <button
      class="navBtn"
      @click="router.push('/')"
      :style="active == 0 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">home</span>
      <p class="select-none">Home</p>
    </button>
    <button
      class="navBtn"
      @click="router.push('/overview')"
      :style="active == 1 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">dashboard</span>
      <p>Overview</p>
    </button>
    <button
      class="navBtn"
      @click="router.push('/network')"
      :style="active == 4 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">hub</span>
      <p class="">Network</p>
    </button>
    <button
      class="navBtn"
      @click="router.push('/connections')"
      :style="active == 2 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">lan</span>
      <p>Conns</p>
    </button>
    <button
      class="navBtn"
      @click="router.push('/protocols')"
      :style="active == 3 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">apps</span>
      <p>Apps</p>
    </button>
    <button
      class="navBtn"
      @click="router.push('/settings')"
      :style="active == 4 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">settings</span>
      <p class="">Settings</p>
    </button>
    <button @click="()=>console.log($route.fullPath)">Show Full Path</button>
  </div>
</template>
<style scoped>
.navBtn {
  height: 4rem;
  padding: 0.5rem 0px;
  user-select: none;
}
.navBtn p {
  margin: 0px;
}
</style>
