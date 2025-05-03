<script setup lang="ts">
import { ref } from 'vue';
import { Protocol as ProtocolType } from '../types';


const props = defineProps({
    protocols: {
        type: Array<[String, ProtocolType]>,
        required: true
    },
    groupName: String,
    isRoot: Boolean,
})

let distinct_children_name_map: Map<string, Array<[String, ProtocolType]>> = new Map();

const toggle = ref(false);


let protocols_to_parse: Array<[String, ProtocolType]> = [...props.protocols];
let final_protos: Array<[String, ProtocolType]> | null = null;

if (props.groupName === "" || props.groupName === "/") {
    final_protos = props.protocols
} else {
    // process distinct names
    for (let [pending_group_name, protocol] of protocols_to_parse) {
        let next_slash = pending_group_name.slice(1).indexOf("/");       // Get the next index of '/', will be -1 if not found(last section)
        let group_name;
        let next_to_be_grouped_name;
        if (next_slash === -1) {
            group_name = pending_group_name.slice(1) // Not found, itself will be a group that contains no slashes
            next_to_be_grouped_name = "/";
        } else {
            group_name = pending_group_name.slice(1, next_slash + 1) // Found, extract the first section to be the group name
            next_to_be_grouped_name = pending_group_name.slice(next_slash + 1)
        }

        if (!distinct_children_name_map.get(group_name)) {

            distinct_children_name_map.set(group_name, [[next_to_be_grouped_name, protocol]])
        } else {
            distinct_children_name_map.get(group_name)!.push([next_to_be_grouped_name, protocol])
        }
    }
}
</script>


<template>
    <section v-if="!(props.groupName === '')" class="border border-gray-200">
        <p class="border-l w-full px-2 py-1 border border-gray-200 shadow-sm" @click="() => toggle = !toggle">{{
            props.groupName }}</p>
        <template v-for="[group_name, protocols] in distinct_children_name_map">
            <div class="pl-4 my-2" v-if="toggle">
                <Protocol :protocols="protocols" :group-name="group_name" />
            </div>
        </template>
    </section>
    <template v-else>
        <ul>
            <li v-for="[_name, proto] in final_protos" class=" w-fit">
                <section class="pl-2 flex gap-6">
                    <p v-if="proto.description?.alias">Name: {{ proto.description.alias }}</p>
                    <p>Protocol String: {{ proto.name }}</p>
                    <p>Protocol Version: {{ proto.major_version }}.{{ proto.minor_version }}.{{ proto.patch_version }}
                    </p>
                </section>
            </li>
        </ul>
    </template>
</template>

<style scoped></style>