<script setup lang="ts">
import GenericAppStatus from "./GenericAppStatus.vue";
</script>

<template>
  <ul class="w-full h-full p-4 select-none">
    <li class="p-2">
      <GenericAppStatus app-name="Messaging" plugin-name="plugin:owlnest-messaging" alternative-path="/app/messaging"
        :show-num-peers="true"></GenericAppStatus>
    </li>
    <li class="p-2">
      <GenericAppStatus app-name="File Transfer" plugin-name="plugin:owlnest-blob-transfer"
        alternative-path="/app/blob-transfer" :show-num-peers="true"></GenericAppStatus>
    </li>
    <li class="p-2">
      <GenericAppStatus app-name="Gossip-Sub Messaging" pluginName="plugin:owlnest-gossipsub"
        alternative-path="/app/gossipsub" />
    </li>
  </ul>
</template>
