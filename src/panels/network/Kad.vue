<script setup lang="ts">
import { RouterView } from "vue-router";
import Tab from "../../components/tab/Tab.vue";
</script>
<template>
  <ul class="tab-container">
    <Tab path="/main/network/kad/peers">Peers</Tab>
    <Tab path="'/main/network/kad/query'">Query</Tab>
    <Tab path="/main/network/kad/bootstrap"></Tab>
  </ul>
  <section style="height: calc(100vh - 2.5rem)" class="overflow-auto">
    <RouterView v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </section>
</template>
