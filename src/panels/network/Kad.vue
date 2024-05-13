<script setup>
import { RouterView } from "vue-router";
</script>
<template>
  <ul class="tab-container">
    <li
      @click="() => $router.push('/main/network/kad/peers')"
      :class="$route.path == '/main/network/kad/peers' ? 'selected' : ''"
    >
      <p>Peers</p>
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
  <section style="height: calc(100vh - 2.5rem);" class="overflow-auto">
    <RouterView v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </section>
</template>
