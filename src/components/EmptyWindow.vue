<template>
    <v-app>
        <RouterView v-slot="{ Component }">
            <component :is="Component" />
        </RouterView>
    </v-app>
</template>