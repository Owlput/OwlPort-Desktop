<script setup></script>
<template>
  <RouterView v-slot="{ Component }"
    ><component :is="Component"></component
  ></RouterView>
</template>
