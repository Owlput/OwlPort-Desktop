<script setup lang="ts">
import Tab from '../../components/tab/Tab.vue';
</script>

<template>
  <ul class="tab-container">
    <Tab path="/main/connections/dial">
      <p>Dial</p>
    </Tab>
    <Tab path="/main/connections/listen">
      <p>Listen</p>
    </Tab>
    <Tab path="/main/connections/connected">
      <p>Connected</p>
    </Tab>
    <Tab path="/main/connections/relays">
      <p>Relays</p>
    </Tab>
  </ul>
  <div style="height: calc(100vh - 2.5rem); width: calc(100vw - 5rem);">
    <RouterView v-slot="{ Component }">
      <component :is="Component" />
    </RouterView>
  </div>
</template>
