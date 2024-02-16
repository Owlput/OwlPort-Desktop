<script setup>
</script>

<template>
  <ul class="tab-l-container">
    <li
      :style="
        $route.path.slice(0, 22) === '/main/connections/dial'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/main/connections/dial')"
    >
      Dial
    </li>
    <li
      :style="
        $route.path.slice(0, 24) === '/main/connections/listen'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/main/connections/listen')"
    >
      Listen
    </li>
    <li
      :style="
        $route.path.slice(0, 27) === '/main/connections/connected'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/main/connections/connected')"
    >
      Connected
    </li>
    <li
      :style="
        $route.path.slice(0, 25) === '/main/connections/relays'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/main/connections/relays')"
    >
      Relays
    </li>
  </ul>
  <div class="w-[100%]" style="height: calc(100vh - 2.5rem)">
    <RouterView v-slot="{ Component }">
      <component :is="Component" />
    </RouterView>
  </div>
</template>
