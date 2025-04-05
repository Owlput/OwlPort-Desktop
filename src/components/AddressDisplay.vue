<script setup lang="ts">
import { is_ip_private } from '../utils';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const props = defineProps({
    address: {
        type: String,
        required: true
    }
});

const PUBLIC_ADDRESS_COLOR = {
    primary: "#64B5F6",
    secondary: "#64B5F6",
}
const PRIVATE_ADDRESS_COLOR = {
    primary: "#81C784",
    secondary: "#81C784",
}
const UNKNOWN_ADDRESS_COLOR = {
    primary: "#616161",
    secondary: "#616161",
}
function check_address_type(addr: string): [{ primary: String, secondary: String, }, String] {
    let components = addr.split('/');
    if (components[1] !== 'ip4' && components[1] !== 'ip6') return [ UNKNOWN_ADDRESS_COLOR, "Unknown address type."];
    if (is_ip_private(components[2])) return [ PRIVATE_ADDRESS_COLOR, "Private address."];
    else return [ PUBLIC_ADDRESS_COLOR, "Public address."]
}
let [ color, tooltip] = check_address_type(props.address);
</script>

<template>
    <v-chip @click="writeText(props.address)" class="select-none" :style="'color: ' + color.primary + ';'">
        {{ props.address }}
        <v-tooltip activator="parent" location="bottom"> {{ tooltip + " Click to copy." }} </v-tooltip>
    </v-chip>
</template>