<script setup>
import { onActivated, ref, computed } from "vue";
import { RouterView, useRoute } from "vue-router";
defineOptions({
  name: "Messaging",
});
let route = useRoute();
const chats = ref([]);
const filtered_chats = computed(() => {
  return chats.value.filter((v) => v.includes(peer_search.value));
});
const peer_search = ref("");
onActivated(() => {
  if (!chats.value.includes(route.params.id) && route.params.id) {
    chats.value.push(route.params.id);
  }
});
</script>

<template>
  <main class="h-[100vh] w-[100vw] grid" id="messaging-grid">
    <section class="border-r w-full">
      <input
        v-model="peer_search"
        class="w-full rounded-none"
        placeholder="Search for a peer..."
      />
      <ul class="flex" v-if="!peer_search">
        <li v-for="chat in chats" class="min-w-[2rem]">
          <button
            class="w-full"
            :style="chat == route.params.id ? { backgroundColor: '#ddd' } : {}"
            @click="
              () => {
                $router.push(`/app/messaging/${chat}`);
              }
            "
          >
            {{ chat.slice(-9, -1) }}
          </button>
        </li>
      </ul>
      <ul class="flex" v-else>
        <li v-if="filtered_chats?.length < 1">No chat fits your criteria</li>
        <li v-for="chat in filtered_chats" class="min-w-[2rem]">
          <button
            class="w-full"
            :style="chat == route.params.id ? { backgroundColor: '#ddd' } : {}"
            @click="
              () => {
                $router.push(`/app/messaging/${chat}`);
              }
            "
          >
            {{ chat.slice(-9, -1) }}
          </button>
        </li>
      </ul>
    </section>
    <section class="w-full h-full">
      <RouterView v-slot="{ Component }">
        <component :is="Component" :key="$route.params.id" />
      </RouterView>
    </section>
  </main>
</template>
<style scoped>
#messaging-grid {
  grid-template-columns: 10rem auto;
}
</style>
