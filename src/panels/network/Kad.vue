<script setup>
import { RouterView } from "vue-router";
</script>
<template>
  <ul class="tab-container">
    <li
      @click="() => $router.push('/main/network/kad/activity')"
      :class="$route.path == '/main/network/kad/activity' ? 'selected' : ''"
    >
      <p>Activity</p>
    </li>
    <li
      @click="() => $router.push('/main/network/kad/query')"
      :class="$route.path.slice(0,23) == '/main/network/kad/query' ? 'selected' : ''"
    >
      <p>Query</p>
    </li>
    <li
      @click="() => $router.push('/main/network/kad/bootstrap')"
      :class="$route.path == '/main/network/kad/bootstrap' ? 'selected' : ''"
    >
      <p>Bootstrap</p>
    </li>
  </ul>
  <section style="height: calc(100vh - 4rem)">
    <RouterView v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </section>
</template>
