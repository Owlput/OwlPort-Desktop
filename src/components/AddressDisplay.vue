<script setup lang="ts">
import { is_ip_private } from '../utils';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import ToolTip from './ToolTip.vue';

const props = defineProps({
    address: {
        type: String,
        required: true
    }
});
enum AddressType {
    Public,
    Private,
    NotApplicable,
}
function check_address_type(addr: string): [AddressType, String] {
    let components = addr.split('/');
    if (components[1] !== 'ip4' && components[1] !== 'ip6') return [AddressType.NotApplicable, "Cannot determine address type"];
    if (is_ip_private(components[2])) return [AddressType.Private, "Private address"];
    else return [AddressType.Public, "Public address"]
}
let [address_type, tooltip] = check_address_type(props.address);
</script>

<template>
    <ToolTip :help-text="tooltip.valueOf()">
        <div @dblclick="writeText(props.address)">
            <section v-if="address_type === AddressType.Public" class="p-1 bg-blue-300 hover:bg-blue-400">
                <p>{{ props.address }}</p>
            </section>
            <section v-else-if="address_type == AddressType.Private" class="p-1 bg-green-300 hover:bg-green-400">
                <p>{{ props.address }}</p>
            </section>
            <section v-else class="p-1 bg-gray-300 hover:bg-slate-400">
                <p>{{ props.address }}</p>
            </section>
        </div>
    </ToolTip>
</template>