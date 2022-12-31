<script setup>
import { ref } from 'vue';
import DisabledPopups from '../../components/DisabledPopups.vue';

const showNestConnectionSettings = ref(false)

function changeNestAddr(addr) {}
</script>
<template>
  <div class="px-16 py-4">
    <section class="m-2">
      <button class="w-full" @click="()=>{showNestConnectionSettings = !showNestConnectionSettings}">
        <p class="float-left m-2">Nest Connection Settings</p>
        <span class="material-icons float-right m-2">expand_{{ showNestConnectionSettings?'less':'more' }}</span>
      </button>
    </section>
    <DisabledPopups></DisabledPopups>
  </div>
</template>
