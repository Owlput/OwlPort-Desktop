<script setup></script>
<template>
  <ul class="flex felx-row justify-evenly items-center">
    <li
      class="boorder shadow-sm p-2"
      :style="$route.path == '/network'?{backgroundColor:'#ddd'}:{}"
      @click="
        () => {
          $router.push('/network');
        }
      "
    >
      Overview
    </li>
    <li
      class="border shadow-sm p-2"
      :style="$route.path == '/network/mdns'?{backgroundColor:'#ddd'}:{}"
      @click="
        () => {
          $router.push('/network/mdns');
        }
      "
    >
      mDNS
    </li>
    <li
      class="border shadow-sm p-2"
      :style="$route.path == '/network/kad'?{backgroundColor:'#ddd'}:{}"
      @click="
        () => {
          $router.push('/network/kad');
        }
      "
    >
      Kad
    </li>
  </ul>
  <RouterView v-slot="{ Component }"
    ><component :is="Component"></component
  ></RouterView>
</template>
