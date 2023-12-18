<script setup>
import { watch, ref, onActivated, onUnmounted } from "vue";
const props = defineProps({
  history: Array,
  remote: String,
});
let will_scroll = ref(true);
let remaining_pos = ref(0);
let show_scroll_bottom = ref(false);
defineEmits(["update:history"]);
watch(props.history, () => {
  if (!will_scroll) {
    return;
  }
  let element = document.getElementById(`chat${props.remote}`);
  console.log(element.scrollHeight - element.scrollTop);
  if (element.scrollHeight - element.scrollTop - 300 < 200) {
    setTimeout(
      () => document.getElementById("last-message")?.scrollIntoView(),
      5
    );
  } else {
    show_scroll_bottom.value = true;
  }
});
watch(remaining_pos, () => {
  if (remaining_pos.value < 120) {
    show_scroll_bottom.value = false;
  }
});
addEventListener("wheel", detect_scroll);
onUnmounted(() => {
  removeEventListener("wheel", detect_scroll);
});
onActivated(() => {
  document.getElementById("last-message")?.scrollIntoView();
});
function detect_scroll(ev) {
  let element = document.getElementById(`chat${props.remote}`);
  remaining_pos.value = element.scrollHeight - element.scrollTop - 350;
}
function scroll_to_bottom() {
  document.getElementById("last-message")?.scrollIntoView();
  show_scroll_bottom.value = false;
}
</script>

<template>
  <div class="relative h-full">
    <section
      class="w-full h-full overflow-auto"
      :id="`chat${props.remote}`"
    >
      <ul class="flex flex-col px-4 py-2">
        <template v-for="message in props.history">
          <li
            v-if="message.from"
            class="message-box bg-gray-300 self-start whitespace-pre-wrap"
            :id="
              props.history[props.history.length - 1] == message
                ? 'last-message'
                : ''
            "
          >
            {{ message.msg }}
          </li>
          <li
            v-else
            class="message-box bg-green-300 self-end whitespace-pre-wrap"
            :id="
              props.history[props.history.length - 1] == message
                ? 'last-message'
                : ''
            "
          >
            {{ message.msg }}
          </li>
        </template>
      </ul>
    </section>
    <button
      v-if="show_scroll_bottom"
      class="absolute bottom-0 right-4 bg-transparent shadow-none border-none"
      @click="scroll_to_bottom"
    >
      <span class="material-icons"> arrow_downward </span>
    </button>
  </div>
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
