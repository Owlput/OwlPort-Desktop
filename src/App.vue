<script setup>
import {useRouter} from 'vue-router'
const router = useRouter();
document.addEventListener('keypress',(ev)=>{
  if (ev.key === "Esc"){
    router.back()
  }
})
</script>

<template>
  <RouterView v-slot="{ Component }">
    <KeepAlive exclude="Overview">
      <component :is="Component" />
    </KeepAlive>
  </RouterView>
</template>