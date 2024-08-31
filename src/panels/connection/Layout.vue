<script setup lang="ts"></script>

<template>
  <ul class="tab-container">
    <li
      :class="
        $route.path.slice(0, 22) === '/main/connections/dial' ? 'selected' : ''
      "
      @click="() => $router.push('/main/connections/dial')"
    >
      <p>Dial</p>
    </li>
    <li
      :class="
        $route.path.slice(0, 24) === '/main/connections/listen'
          ? 'selected'
          : ''
      "
      @click="() => $router.push('/main/connections/listen')"
    >
      <p>Listen</p>
    </li>
    <li
      :class="
        $route.path.slice(0, 27) === '/main/connections/connected'
          ? 'selected'
          : ''
      "
      @click="() => $router.push('/main/connections/connected')"
    >
      <p>Connected</p>
    </li>
    <li
      :class="
        $route.path.slice(0, 25) === '/main/connections/relays'
          ? 'selected'
          : ''
      "
      @click="() => $router.push('/main/connections/relays')"
    >
      <p>Relays</p>
    </li>
  </ul>
  <div class="w-[100%]" style="height: calc(100vh - 2.5rem)">
    <RouterView v-slot="{ Component }">
      <component :is="Component" />
    </RouterView>
  </div>
</template>
