<script setup>
import { invoke } from "@tauri-apps/api";
import { ref, onUnmounted } from "vue";

const num_connected = ref(0);
function update_display() {
  invoke("plugin:owlnest-swarm|list_connected").then(
    (result) => (num_connected.value = result.length)
  );
}
update_display();
let interval_id = setInterval(update_display, 5000);
onUnmounted(() => {
  clearInterval(interval_id);
});
</script>
<template>
  <main class="card-wrapper">
    <section class="relative">
      <p>Peers Connected</p>
      <p class="absolute bottom-0 text-[2rem] w-full my-4">
        {{ num_connected }}
      </p>
    </section>
  </main>
</template>
<style scoped>
.wrapper {
  margin: 0.5rem;
  padding: 0.25rem;
  height: fit-content;
  width: fit-content;
}
</style>
