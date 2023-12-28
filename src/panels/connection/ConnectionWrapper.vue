<script setup>
defineOptions({
  name: "ConnectionWrapper",
});
</script>

<template>
  <ul class="tab-l-container">
    <li
      :style="
        $route.path.slice(0, 17) === '/connections/dial'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/connections/dial')"
    >
      Dial
    </li>
    <li
      :style="
        $route.path.slice(0, 19) === '/connections/listen'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/connections/listen')"
    >
      Listen
    </li>
    <li
      :style="
        $route.path.slice(0, 22) === '/connections/connected'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/connections/connected')"
    >
      Connected
    </li>
    <li
      :style="
        $route.path.slice(0, 20) === '/connections/relays'
          ? 'background-color:gainsboro;'
          : ''
      "
      @click="() => $router.push('/connections/relays')"
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
