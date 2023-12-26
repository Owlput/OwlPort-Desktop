<script setup>
import { onActivated, ref } from "vue";
import { RouterView, useRoute } from "vue-router";
defineOptions({
  name: "Messaging",
});
let route = useRoute();
let active_chat = ref(route.params.id?[route.params.id]:[]);
onActivated(() => {
  if (!active_chat.value.includes(route.params.id) && route.params.id) {
    active_chat.value.push(route.params.id);
  }
});
</script>

<template>
  <div class="h-[100vh] w-full">
    <section class="border-b shadow-md py-2 h-10">
      <button @click="() =>$router.push('/protocols')" class="float-left mx-2">
        Back
      </button>
      <ul class="flex">
        <li v-for="chat in active_chat" class="min-w-[2rem]">
          <button
            class="w-full"
            :style="chat == route.params.id?{ backgroundColor:'#ddd' }:{ }"
            @click="
              () => {
                $router.push(`/protocols/messaging/${chat}`);
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
        <component :is="Component" :key="$route.params.id"/>
      </KeepAlive>
    </RouterView>
  </div>
</template>