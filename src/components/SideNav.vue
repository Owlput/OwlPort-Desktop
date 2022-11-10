<script setup>
import { ref } from "vue";

const props = defineProps({
  showDesp: Boolean,
});

const tabActive = ref("home");
const alert = () => {
  console.log("dblclicked");
};
</script>
<template>
  <QTabs v-model="tabActive" vertical>
    <button
      class="h-16 w-full button-no-style-override"
      style="border: none; box-shadow: none; border-radius: 0px"
      @dblclick="
        (e) => {
          e.preventDefault();
          alert();
        }
      "
    >
      <p class="m-0">v0.0.0</p>
    </button>
    <QRouteTab name="home" icon="home" to="/">
      <p class="m-0" v-if="props.showDesp">Home</p>
    </QRouteTab>
    <QRouteTab name="overview" icon="dashboard" to="/overview"
      ><p class="m-0 overflow-hidden" v-if="props.showDesp">
        Overview
      </p></QRouteTab
    >
    <QRouteTab name="tab2" icon="build"></QRouteTab>
    <QRouteTab name="tab3" icon="build" to="/settings"><p class="m-0 overflow-hidden" v-if="props.showDesp">
        Settings
      </p></QRouteTab>
  </QTabs>
</template>
