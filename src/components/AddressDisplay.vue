<script setup lang="ts">
import { is_ip_private } from '../utils';

const props = defineProps({
    address:{
        type: String,
        required: true
    }
});
enum AddressType{
  Public,
  Private,
  NotApplicable,
}
function check_address_type(addr:string):AddressType{
  let components = addr.split('/');
  if (components[1] !== 'ip4' && components[1] !== 'ip6') return AddressType.NotApplicable;
  if (is_ip_private(components[2])) return AddressType.Private;
  else return AddressType.Public
}
let address_type = check_address_type(props.address);
</script>

<template>
    <section v-if="address_type === AddressType.Public" class="p-1 bg-blue-300 hover:bg-blue-400">
        <p>{{ props.address }}</p>
    </section>
    <section v-else-if="address_type == AddressType.Private" class="p-1 bg-green-300 hover:bg-green-400">
        <p>{{ props.address }}</p>
    </section>
    <section v-else class="p-1 bg-gray-300 hover:bg-slate-400">
        <p>{{ props.address }}</p>
    </section>
</template>