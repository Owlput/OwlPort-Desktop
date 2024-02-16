<script setup></script>
<template>
  <ul class="tab-l-container">
    <li
      :style="$route.path == '/main/network' ? { backgroundColor: '#ddd' } : {}"
      @click="() => $router.push('/main/network')"
    >
      <p>Overview</p>
    </li>
    <li
      :style="
        $route.path.slice(0, 18) == '/main/network/mdns'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="() => $router.push('/main/network/mdns')"
    >
      mDNS
    </li>
    <li
      :style="
        $route.path.slice(0, 17) == '/main/network/kad'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="() => $router.push('/main/network/kad')"
    >
      Kad
    </li>
    <li
      :style="
        $route.path.slice(0, 21) == '/main/network/autonat'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="() => $router.push('/main/network/autonat')"
    >
      AutoNAT
    </li>
    <li
      :style="
        $route.path.slice(0, 18) == '/main/network/upnp'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="() => $router.push('/main/network/upnp')"
    >
      UPnP
    </li>
  </ul>
  <RouterView v-slot="{ Component }"
    ><component :is="Component"></component
  ></RouterView>
</template>
