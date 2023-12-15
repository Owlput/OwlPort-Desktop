<script setup>
const props = defineProps({
  message_history: Array,
});
</script>

<template>
  <section class="h-full w-full">
    <ul class=" overflow-auto flex flex-col px-4 py-2">
      <template v-for="message in props.message_history">
        <li
          v-if="message.from"
          class="message-box bg-gray-300 self-start whitespace-pre-wrap"
        >
          {{ message.msg }}
        </li>
        <li v-else class="message-box bg-green-300 self-end whitespace-pre-wrap">
          {{ message.msg }}
        </li>
      </template>
    </ul>
  </section>
</template>
<style>
.message-box {
  padding: 0.25rem;
  border-radius: 0.25rem;
  border: 1px solid black;
  width: max-content;
  max-width: 65%;
  min-height: 2rem;
  margin: 0.25rem 0px;
}
</style>
