<script setup></script>
<template>
  <ul class="flex felx-row justify-between items-center">
    <li
      class="boorder shadow-sm"
      @click="
        () => {
          $router.push('/network');
        }
      "
    >
      Overview
    </li>
    <li
      class="border shadow-sm"
      @click="
        () => {
          $router.push('/network/mdns');
        }
      "
    >
      mDNS
    </li>
  </ul>
  <RouterView v-slot="{ Component }"
    ><component :is="Component"></component
  ></RouterView>
</template>
