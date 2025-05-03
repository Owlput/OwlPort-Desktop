<script setup lang="ts">
</script>
<template>
  <div class="w-full flex flex-col bg-white shadow-md h-[100vh]">
    <button class="navBtn" @click="$router.push('/app/messaging')" :style="$route.path === '/app/messaging'
      ? 'background-color:gainsboro;'
      : ''
      ">
      <span class="mdi-mailbox mdi text-3xl"></span>
      <p class="select-none">Inbox</p>
    </button>
    <button class="navBtn" @click="$router.push('/app/messaging/chat')" :style="$route.path.slice(0, 19) === '/app/messaging/chat'
      ? 'background-color:gainsboro;'
      : ''
      ">
      <span class="mdi-message-outline mdi text-3xl"></span>
      <p>Chat</p>
    </button>
    <button class="navBtn" @click="$router.push('/app/messaging/contacts')" :style="$route.path.slice(0, 23) === '/app/messaging/contacts'
      ? 'background-color:gainsboro;'
      : ''
      ">
      <span class="mdi-contacts-outline mdi text-3xl"></span>
      <p class="">Contacts</p>
    </button>
    <button class="navBtn" @click="$router.push('/app/messaging/discover')" :style="$route.path.slice(0, 23) === '/app/messaging/discover'
      ? 'background-color:gainsboro;'
      : ''
      ">
      <span class="mdi-web mdi text-3xl"></span>
      <p>Discover</p>
    </button>
    <button class="navBtn" @click="$router.push('/app/messaging/settings')" :style="$route.path.slice(0, 23) === '/app/messaging/settings'
      ? 'background-color:gainsboro;'
      : ''
      ">
      <span class="mdi-cog mdi text-3xl"></span>
      <p>Settings</p>
    </button>
    <!-- <button @click="()=>console.log($route.fullPath)">Show Full Path</button> -->
  </div>
</template>
<style scoped>
.navBtn {
  height: 4.5rem;
  padding: 0.5rem 0.25px;
  user-select: none;
  margin: 0px;
  border: none;
  box-shadow: none;
  border-radius: 0px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-evenly;
}

.navBtn:hover {
  background-color: gainsboro;
}

.navBtn p {
  margin: 0px;
}
</style>
