<script setup>
const props = defineProps({
  showDesp: Boolean,
});
</script>
<template>
  <div class="w-fit flex flex-col bg-white shadow-md h-[100vh]">
    <div class="mx-auto">
      <p>v0.0.0</p>
    </div>
    <button
      class="navBtn"
      @click="$router.push('/')"
      :style="$route.path.length === 1 && $route.path.slice(0,1) === '/' ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">home</span>
      <p class="select-none">Home</p>
    </button>
    <button
      class="navBtn"
      @click="$router.push('/overview')"
      :style="$route.path.slice(0,9) === '/overview' ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">dashboard</span>
      <p>Overview</p>
    </button>
    <button
      class="navBtn"
      @click="$router.push('/network')"
      :style="$route.path.slice(0,8) === '/network' ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">hub</span>
      <p class="">Network</p>
    </button>
    <button
      class="navBtn"
      @click="$router.push('/connections')"
      :style="$route.path.slice(0,12) === '/connections' ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">lan</span>
      <p>Conns</p>
    </button>
    <button
      class="navBtn"
      @click="$router.push('/protocols')"
      :style="$route.path.slice(0,10) === '/protocols' ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">apps</span>
      <p>Apps</p>
    </button>
    <button
      class="navBtn"
      @click="$router.push('/settings')"
      :style="$route.path.slice(0,9) === '/settings' ? 'background-color:gainsboro;' : ''"
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
