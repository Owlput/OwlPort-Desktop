<script setup>
import { RouterView } from "vue-router";
</script>
<template>
  <ul class="tab-m-container">
    <li
      @click="() => $router.push('/main/network/kad/activity')"
      :style="$route.path == '/main/network/kad/activity' ? { backgroundColor: '#ddd' } : {}"
    >
      <p>Activity</p>
    </li>
    <li
      @click="() => $router.push('/main/network/kad/query')"
      :style="$route.path.slice(0,23) == '/main/network/kad/query' ? { backgroundColor: '#ddd' } : {}"
    >
      <p>Query</p>
    </li>
    <li
      @click="() => $router.push('/main/network/kad/bootstrap')"
      :style="$route.path == '/main/network/kad/bootstrap' ? { backgroundColor: '#ddd' } : {}"
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
