<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, onUnmounted, ref, shallowRef } from "vue";
import FailedHeartbeat from "../components/popups/FailedHeartbeat.vue";

const intervals = ref([]);
onMounted(() => {
  hb();
  let hbHandle = setInterval(hb, 1000);
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
      let stamp = Date.now()
      dispatchEvent(
        new CustomEvent("createPopup", {
          detail: {
            handlerType: "hb-fail",
            timeout:5000,
            stamp,
            popup:
            {
            component:shallowRef(FailedHeartbeat),
            props:{
              reason:e,
              dismiss:() =>{
                  console.warn(
                    "You've disabled pop-ups for failing heartbeat. You may not receive further notifications when situation changes."
                  );
                  dispatchEvent(
                    new CustomEvent("disablePopup", {detail: { handlerType: "hb-fail" },}));
                  dispatchEvent(new CustomEvent("deletePopup",{detail:{stamp}}))
                },
              retry:()=>{reconnect();dispatchEvent(new CustomEvent("deletePopup",{detail:{stamp}}))}
            }}
          },
        })
      );
    });
}
</script>
<template></template>
