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
      msg.value = v;
      dispatchEvent(
        new CustomEvent("enablePopup", { detail: { typeHandler: "hb-fail" } })
      );
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
            position: "bottom-right",
            message: `Cannot send heartbeat to nest: 
          ${e}`,
            typeHandler: "hb-fail",
            progress: true,
            timeout:5000,
            actions: [
              {
                label: "Dismiss",
                handler: () =>{
                  console.warn(
                    "You're disabling pop-ups for failing heartbeat. You may not receive further notifications when things change."
                  );
                  dispatchEvent(
                    new CustomEvent("disablePopup", {
                      detail: { typeHandler: "hb-fail" },
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
