<script setup>
import { RouterView } from "vue-router";
</script>
<template>
  <ul class="tab-m-container">
    <li
      @click="() => $router.push('/network/kad/activity')"
      :style="$route.path.length == 12 || $route.path.slice(0,21) == '/network/kad/activity' ? { backgroundColor: '#ddd' } : {}"
    >
      <p>Activity</p>
    </li>
    <li
      @click="() => $router.push('/network/kad/query')"
      :style="$route.path.slice(0,18) == '/network/kad/query' ? { backgroundColor: '#ddd' } : {}"
    >
      <p>Query</p>
    </li>
    <li
      @click="() => $router.push('/network/kad/bootstrap')"
      :style="$route.path == '/network' ? { backgroundColor: '#ddd' } : {}"
    >
      <p>Bootstrap</p>
    </li>
  </ul>
  <section style="height: calc(100vh - 5.5rem)">
    <RouterView v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </section>
</template>
