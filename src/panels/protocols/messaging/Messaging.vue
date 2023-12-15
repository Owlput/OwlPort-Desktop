<script setup>
import { ref } from "vue";
import { RouterView } from "vue-router";
defineOptions({
  name: "Messaging",
});
let active_chat = ref([]);
function new_chat(peer_id) {
  active_chat.value.push({
    label: active_chat.value.length + 1,
    peer_id: peer_id,
  });
}
</script>

<template>
  <div class="h-full" id="wrapper">
  <section class="border-b shadow-md py-2">
    <button
      @click="
        () => new_chat('12D3KooWJSyUkSRHetgW2dVMx9SNP2g7YWYSzgFZTeRAEDWV5UGN')
      "
      class="float-left mx-2"
    >
      New Chat
    </button>
    <ul class="flex">
      <li v-for="chat in active_chat" class=" min-w-[2rem]">
        <button class="w-full"
          @click="
            () => {
              $router.push(`/protocols/messaging/${chat.peer_id}`);
            }
          "
        >
          {{ chat.label }}
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
#wrapper{
  display: grid;
  grid-template-rows: 4rem auto;
}
</style>