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
    <button class="tabBtn" @click="router.push('/')">
      <span class="material-icons m-auto">home</span>
    </button>
    <button class="tabBtn" @click="router.push('/overview')">
      <span class="material-icons m-auto">dashboard</span>
    </button>
    <button class="tabBtn" @click="router.push('/settings')">
      <span class="material-icons m-auto">settings</span>
    </button>
  </div>
</template>
<style scoped>
.tabBtn{
  margin: 0px;
  border: none;
  box-shadow: none;
  border-radius: 0px;
  height:4rem;
}
.tabBtn:hover{
  background-color: gainsboro;
}
</style>