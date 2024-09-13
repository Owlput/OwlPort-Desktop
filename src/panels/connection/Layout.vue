<script setup lang="ts">
import Tab from '../../components/tab/Tab.vue';
</script>

<template>
  <ul class="tab-container">
    <Tab path="/main/connections/dial"><p class="p-[0.375rem]">Dial</p></Tab>
    <Tab path="/main/connections/listen"><p class="p-[0.375rem]">Listen</p></Tab>
    <Tab path="/main/connections/connected"><p class="p-[0.375rem]">Connected</p></Tab>
    <Tab path="/main/connections/relays"><p class="p-[0.375rem]">Relays</p></Tab>
  </ul>
  <div style="height: calc(100vh - 2.5rem); width: calc(100vw - 5rem);">
    <RouterView v-slot="{ Component }">
      <component :is="Component" />
    </RouterView>
  </div>
</template>
