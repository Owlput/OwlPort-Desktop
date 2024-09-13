<script setup lang="ts">
const is_bodyless = !import.meta.env.VITE_WITH_RUST;
</script>
<template>
  <div class="w-full flex flex-col bg-white shadow-md h-[100vh]">
    <div class="mx-auto text-center">
      <p>v0.0.0</p>
      <p v-if="is_bodyless">Bodyless</p>
    </div>
    <button class="navBtn" @click="$router.push('/main/home')" :style="$route.path.slice(0, 11) === '/main/home'
        ? 'background-color:gainsboro;'
        : ''
      ">
      <span class="material-icons m-auto">home</span>
      <p class="select-none">Home</p>
    </button>
    <button class="navBtn" @click="$router.push('/main/overview')" :style="$route.path.slice(0, 14) === '/main/overview'
        ? 'background-color:gainsboro;'
        : ''
      ">
      <span class="material-icons m-auto">dashboard</span>
      <p>Overview</p>
    </button>
    <button class="navBtn" @click="$router.push('/main/network')" :style="$route.path.slice(0, 13) === '/main/network'
        ? 'background-color:gainsboro;'
        : ''
      ">
      <span class="material-icons m-auto">hub</span>
      <p class="">Network</p>
    </button>
    <button class="navBtn" @click="$router.push('/main/connections')" :style="$route.path.slice(0, 17) === '/main/connections'
        ? 'background-color:gainsboro;'
        : ''
      ">
      <span class="material-icons m-auto">lan</span>
      <p>Conns</p>
    </button>
    <button class="navBtn" @click="$router.push('/main/apps')" :style="$route.path.slice(0, 11) === '/main/apps'
        ? 'background-color:gainsboro;'
        : ''
      ">
      <span class="material-icons m-auto">apps</span>
      <p>Apps</p>
    </button>
    <button class="navBtn" @click="$router.push('/main/settings')" :style="$route.path.slice(0, 14) === '/main/settings'
        ? 'background-color:gainsboro;'
        : ''
      ">
      <span class="material-icons m-auto">settings</span>
      <p class="">Settings</p>
    </button>
    <!-- <button @click="()=>console.log($route.fullPath)">Show Full Path</button> -->
  </div>
</template>
<style scoped>
.navBtn {
  height: 4rem;
  padding: 0.5rem 0px;
  user-select: none;
  margin: 0px;
  border: none;
  box-shadow: none;
  border-radius: 0px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-evenly;
}

.navBtn:hover {
  background-color: gainsboro;
}

.navBtn p {
  margin: 0px;
}
</style>
