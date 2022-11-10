<script setup>
    import {ref} from "vue"
    const routeActive = ref("general");
</script>
<template>
    <QTabs v-model="routeActive">
        <QRouteTab name="general" to="/settings/general">General</QRouteTab>
        <QRouteTab name="certificates" to="/settings/certificates">Certificates</QRouteTab>
    </QTabs>
    <RouterView></RouterView>
</template>