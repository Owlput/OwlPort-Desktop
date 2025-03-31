<script setup lang="ts">
import { Ref, ref } from 'vue';
const props = defineProps({
    helpText: {
        type: String,
        required: true
    },
    delay: {
        type: Number,
        default: 750
    },
    top: Boolean,
    bottom: Boolean,
    left: Boolean,
    right: Boolean,
})

const show_tooltop = ref(false)
const timeout: Ref<number | null> = ref(null);
function on_enter() {
    timeout.value = setTimeout(() => {
        show_tooltop.value = true
    }, props.delay)
}
function on_leave() {
    if (timeout.value !== null) clearInterval(timeout.value);
    show_tooltop.value = false;
}
let postioning = " top-[100%] left-[50%] translate-x-[-50%]"
if (props.left) postioning = " top-[50%] left-0 translate-y-[-50%]";
if (props.right) postioning = " top-[50%] right-0 translate-x-[-50%]";
if (props.top) postioning = " bottom-[100%] left-[50%] translate-x-[-50%]";
if (props.bottom) postioning = " top-[100%] left-[50%] translate-x-[-50%]";
</script>
<template>
    <div class="relative" @mouseenter="on_enter" @mouseleave="on_leave">
        <slot></slot>
        <template v-if="show_tooltop">
            <p :class="'absolute text-wrap shadow-md bg-white rounded-md text-sm max-w-[200%]' + postioning"
                style="padding: 0.125rem; z-index: 20;">{{ props.helpText }}</p>
        </template>
    </div>
</template>