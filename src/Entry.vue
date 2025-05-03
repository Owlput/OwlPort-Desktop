<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();
document.addEventListener("keyup", (ev: KeyboardEvent) => {
    if (!ev.isTrusted || ev.isComposing) {
        return;
    }
    if (ev.key == "Escape") {
        router.back();
    }
});
</script>
<template>
    <v-app>
        <RouterView v-slot="{ Component }">
            <component :is="Component" />
        </RouterView>
    </v-app>
</template>