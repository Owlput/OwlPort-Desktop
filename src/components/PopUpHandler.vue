<script setup>
import { onMounted } from "vue";
import { useQuasar } from "quasar";
const q = useQuasar();

onMounted(() => {
  addEventListener("createPopup", (ev) => {
    if (!window.sessionStorage.getItem(ev.detail.typeHandler)) q.notify(ev.detail);
  });
  addEventListener("disablePopup", (ev) => {
    window.sessionStorage.setItem(ev.detail.typeHandler,"disabled")
    ;
  });
  addEventListener("enablePopup", (ev) => {
    window.sessionStorage.removeItem(ev.detail.typeHandler);
  });
});
</script>
<template></template>