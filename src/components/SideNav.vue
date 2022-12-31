<script setup>
import { ref, computed } from "vue";
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
    case "apps":
      return 2;
    case "settings":
      return 3;
  }
});

const tabActive = ref("home");
</script>
<template>
  <div class="w-[100%] flex flex-col bg-white shadow-md">
    <div class="mx-auto">
      <p>v0.0.0</p>
    </div>
    <button
      class="navBtn"
      @click="router.push('/')"
      :style="active == 0 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">home</span>
      <p>Home</p>
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
      @click="router.push('/apps')"
      :style="active == 2 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">apps</span>
      <p>Apps</p>
    </button>
    <button
      class="navBtn"
      @click="router.push('/settings')"
      :style="active == 3 ? 'background-color:gainsboro;' : ''"
    >
      <span class="material-icons m-auto">settings</span>
      <p>Settings</p>
    </button>
  </div>
</template>
<style scoped>
.navBtn {
  height: 4rem;
  padding: 0.5rem 0px;
}
.navBtn p {
  margin: 0px;
}
</style>
