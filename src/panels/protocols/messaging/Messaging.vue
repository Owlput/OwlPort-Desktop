<script setup>
import { onActivated, ref } from "vue";
import { RouterView, useRoute } from "vue-router";
defineOptions({
  name: "Messaging",
});
let route = useRoute();
let active_chat = ref([route.params.id]);
onActivated(() => {
  if (!active_chat.value.includes(route.params.id)) {
    active_chat.value.push(route.params.id);
  }
});
</script>

<template>
  <div class="h-full" id="wrapper">
    <section class="border-b shadow-md py-2">
      <button @click="() =>$router.push('/protocols')" class="float-left mx-2">
        Back
      </button>
      <ul class="flex">
        <li v-for="chat in active_chat" class="min-w-[2rem]">
          <button
            class="w-full"
            @click="
              () => {
                $router.push(`/protocols/messaging/${chat.peer_id}`);
              }
            "
          >
            {{ chat.slice(-9,-1) }}
          </button>
        </li>
      </ul>
    </section>
    <RouterView v-slot="{ Component }">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </RouterView>
  </div>
</template>
<style scoped>
#wrapper {
  display: grid;
  grid-template-rows: 4rem auto;
}
</style>
