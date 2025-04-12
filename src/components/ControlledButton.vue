<script setup lang="ts">
import { ref, watch } from 'vue';
import { isBodylessHandler } from '../utils';

const click_promise = defineModel('clickpromise');
const is_loading = defineModel('isloading');

const props = defineProps({
    size: String,
    type: String,
    promise_success: Function,
    promise_catch: Function,
    promise_finally: Function,
})

const button_state = ref({
    show_success: false,
    success: false,
})

watch(click_promise, () => {
    let promise = null;
    if (!click_promise.value) return;
    is_loading.value = true;
    [click_promise.value, promise] = [promise, click_promise.value];
    (promise as Promise<any>).then((v) => {
        if (props.promise_success) props.promise_success(v)
        console.log(v)
        is_loading.value = false;
        button_state.value.success = true;
        button_state.value.show_success = true;
        setTimeout(() => {
            button_state.value.show_success = false;
            button_state.value.success = false;
        }, 5000)
    }).catch((e) => {
        if (props.promise_catch) props.promise_catch(e)
        is_loading.value = false;
        isBodylessHandler(e);
        button_state.value.success = false;
        button_state.value.show_success = true;
        setTimeout(() => {
            button_state.value.show_success = false;
            button_state.value.success = false;
        }, 5000)
    }).finally(() => {
        if (props.promise_finally) props.promise_finally()
    })
})

</script>

<template>
    <v-btn :loading="is_loading" :type="props.type" :size="props.size"
        :color="button_state.show_success ? (button_state.success ? '#66BB6A' : '#EF5350') : ''">
        <slot></slot>
    </v-btn>
</template>