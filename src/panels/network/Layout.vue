<script setup></script>
<template>
  <ul class="tab-l-container">
    <li
      :style="$route.path == '/network' ? { backgroundColor: '#ddd' } : {}"
      @click="
        () => {
          $router.push('/network');
        }
      "
    >
      <p>Overview</p>
    </li>
    <li
      :style="
        $route.path.slice(0, 13) == '/network/mdns'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="
        () => {
          $router.push('/network/mdns');
        }
      "
    >
      mDNS
    </li>
    <li
      :style="
        $route.path.slice(0, 12) == '/network/kad'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="
        () => {
          $router.push('/network/kad');
        }
      "
    >
      Kad
    </li>
    <li
      :style="
        $route.path.slice(0, 12) == '/network/autonat'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="
        () => {
          $router.push('/network/autonat');
        }
      "
    >
      AutoNAT
    </li>
    <li
      :style="
        $route.path.slice(0, 12) == '/network/upnp'
          ? { backgroundColor: '#ddd' }
          : {}
      "
      @click="
        () => {
          $router.push('/network/upnp');
        }
      "
    >
      UPnP
    </li>
  </ul>
  <RouterView v-slot="{ Component }"
    ><component :is="Component"></component
  ></RouterView>
</template>
