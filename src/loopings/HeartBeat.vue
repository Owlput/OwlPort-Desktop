<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, onUnmounted, ref } from "vue";

const intervals = ref([]);
onMounted(() => {
  hb();
  let hbHandle = setInterval(hb, 30000);
  intervals.value.push(hbHandle);
});
onUnmounted(() => {
  intervals.value.map((v) => {
    clearInterval(v);
  });
});
const reconnect = async () => await invoke("plugin:grpc_hb|reconnect");

function hb() {
  invoke("plugin:grpc_hb|hb")
    .then((v) => {
      dispatchEvent(
        new CustomEvent("hb-success", {
          detail: {
            hbResponse: v,
          },
        })
      );
    })
    .catch((e) => {
      dispatchEvent(
        new CustomEvent("createPopup", {
          detail: {
            type: "warning",
            handlerType: "hb-fail",
            message: `Cannot send heartbeat to nest:${e}`,
            timeout:5000,
            actions: [
              {
                label: "Dismiss",
                handler: () =>{
                  console.warn(
                    "You've disabled pop-ups for failing heartbeat. You may not receive further notifications when situation changes."
                  );
                  dispatchEvent(
                    new CustomEvent("disablePopup", {
                      detail: { handlerType: "hb-fail" },
                    })
                  );
                },
              },
              {
                label: "Retry",
                handler: reconnect,
              },
            ],
          },
        })
      );
    });
}
</script>
<template></template>
