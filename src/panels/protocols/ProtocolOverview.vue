<script setup>
import {onUnmounted} from "vue"
onUnmounted(()=>{
    console.log("ProtocolOverview destroyed")
})
</script>

<template>
  <ul class="w-full h-full p-4 select-none" v-show="$route.path == '/protocols'">
    <li
      class="flex justify-between items-center h-12 border rounded-lg px-4 hover:bg-slate-100 hover:cursor-pointer active:bg-slate-200"
      @click="
        () => {
          console.log('clicked');
          $router.push('/protocols/messaging');
        }
      "
    >
      <p class="">Messaging</p>
      <p>Peers supported: {{ 1 }}</p>
      <span class="material-icons"> chevron_right </span>
    </li>
  </ul>
  <RouterView v-slot="{Component}"><KeepAlive><component :is="Component"/></KeepAlive></RouterView>
</template>
