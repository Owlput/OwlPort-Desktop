<script setup>
import { computed } from "vue";
import { useRouter } from "vue-router";

let router = useRouter();
const active = computed(() => {
  switch (router.currentRoute.value.path.split("/")[2]) {
    case "":
      return 0;
    case "general":
      return 0;
    case "certs-and-keys":
      return 1;
    default: return 0;
  }
});
</script>
<template>
  <div class="w-full h-10 flex flex-row justify-evenly shadow-sm">
    <button class="navBtn" :style="(active == 0)?'background-color:gainsboro;':''" @click="router.push('/settings/general')">General</button>
    <button class="navBtn" :style="(active == 1)?'background-color:gainsboro;':''" @click="router.push('/settings/certs-and-keys')">Certs and Keys</button>
  </div>
  <RouterView></RouterView>
</template>
<style scoped>
.navBtn {
  padding: 0px 0.5rem;
}
</style>
