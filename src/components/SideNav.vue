<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

let router = useRouter()
const props = defineProps({
  showDesp: Boolean,
});

const tabActive = ref("home");
const alert = () => {
  console.log("dblclicked");
};
</script>
<template>
  <div class="w-[100%] flex flex-col bg-white shadow-md">
    <div class="mx-auto">
      <p>v0.0.0</p>
    </div>
    <button class="" @click="router.push('/')">
      <img src="../assets/icons/home_black_48dp.svg"/>
    </button>
    <button class="" @click="router.push('/overview')">
      <img src="../assets/icons/dashboard_black_48dp.svg"/>
    </button>
    <button class="" @click="router.push('/settings')">
      <img src="../assets/icons/settings_black_48dp.svg"/>
    </button>
  </div>
</template>
