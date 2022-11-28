<script setup>
import { ref } from "vue";
import KeyAndCert from "../utilities/KeyAndCert.vue";
const pathToCertificate = ref("certs/");
const kacUtil = ref(false);
function toggleKacUtil(){
    kacUtil.value = true
    console.log("dialog toggled")
}
const columnDef = [
  {
    name: "name",
    required: true,
    label: "Name",
    align: "left",
    field: (row) => row.name,
    format: (val) => `${val}`,
    sortable: true,
  },
  {
    name: "desp",
    required: true,
    label: "Description",
    align: "left",
    field: (row) => row.name,
    format: (val) => `${val}`,
    sortable: true,
  },
  {
    name: "name",
    required: true,
    label: "Name",
    align: "left",
    field: (row) => row.name,
    format: (val) => `${val}`,
    sortable: true,
  },
];
</script>
<template>
  <QList class="q-pa-md">
    <QExpansionItem
      label="Local Certificates"
      caption="No certificate configured"
      icon="class"
    >
      <QCard class="q-pa-md">
        <QInput
          label="Certificate Folder"
          v-model="pathToCertificate"
          class="w-full"
        ></QInput>
        <QTable></QTable
      ></QCard>
    </QExpansionItem>
    <QExpansionItem
      label="Remote Certificates"
      caption="No certificate configured"
      icon="dns"
    ></QExpansionItem>
  </QList>
  <div class="w-full px-[5%] mt-4">
    <p>Related</p>
    <QBtn unelevated class="full-width" @click="toggleKacUtil">
      <template v-slot:default
        ><div class="w-full">
          <p class="m-0 float-left">Key and Cert Utilities</p>
          <QIcon name="arrow_forward" class="float-right" />
        </div>
      </template>
    </QBtn>
  </div>
  <QDialog v-model="kacUtil"><KeyAndCert></KeyAndCert></QDialog>
</template>
