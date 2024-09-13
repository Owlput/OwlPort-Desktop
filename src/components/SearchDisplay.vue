<script setup lang="ts" generic="T">
import { computed, ComputedRef, PropType, Ref, ref } from 'vue';
import SearchBar from './SearchBar.vue';

type CriteriaFn = (source: Array<T>, search_text: String) => Array<T>;
const props = defineProps({
    criteria: {
        type: Function as PropType<CriteriaFn>,
        required: true,
    },
    get_or_refresh: { type: Function as PropType<() => Promise<Array<T>>>, required: true },
    placeHolder: String,
})

const search_text: Ref<String> = ref('');
const list_items: Ref<Array<T>> = ref([]);
const displayed_items: ComputedRef<Array<T>> = computed(() => {
    if (search_text.value.length === 0) return list_items.value;
    return props.criteria(list_items.value, search_text.value)
})
function refresh() {
    props.get_or_refresh().then(v => list_items.value = v)
}
refresh()
</script>
<template>
    <SearchBar v-model="search_text" :place-holder="props.placeHolder" :refresh="refresh" />
    <ul>
        <template v-for="item in displayed_items">
            <slot :item="item"></slot>
        </template>
    </ul>

</template>