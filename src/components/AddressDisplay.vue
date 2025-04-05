<script setup lang="ts">
import { is_ip_private } from '../utils';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const props = defineProps({
    address: {
        type: String,
        required: true
    },
    behavior: {
        type: String,
        default: "none"
    },
    onClick: {
        type: Function
    },
    onClickDesc: {
        type: String,
        default: ''
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
function check_address_type(addr: string): [{ primary: string, secondary: string, }, string] {
    let components = addr.split('/');
    if (components[1] !== 'ip4' && components[1] !== 'ip6') return [UNKNOWN_ADDRESS_COLOR, "Unknown address type."];
    if (is_ip_private(components[2])) return [PRIVATE_ADDRESS_COLOR, "Private address."];
    else return [PUBLIC_ADDRESS_COLOR, "Public address."]
}
let [color, tooltip] = check_address_type(props.address);
let [on_click, on_click_desc] = check_behavior()
function check_behavior(): [Function, string] {
    switch (props.behavior) {
        case 'none': return [() => { }, '']
        case 'copy': return [() => writeText(props.address), "Click to copy."];
        case 'custom': if (props.onClick) return [props.onClick, props.onClickDesc]
    }
    throw new Error("Invalid click behavior")
}
</script>

<template>
    <v-chip @click="on_click" class="select-none" :style="'color: ' + color.primary + ';'">
        {{ props.address }}
        <v-tooltip activator="parent" location="bottom"> {{ tooltip + ' ' + on_click_desc }} </v-tooltip>
    </v-chip>
</template>