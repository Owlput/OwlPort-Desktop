<script setup lang="ts" generic="T">
import { computed, ComputedRef, PropType, Ref, ref, shallowRef } from 'vue';
import SearchBar from './SearchBar.vue';
import { isBodyless, Status } from '../utils';

type CriteriaFn = (source: T, search_text: string) => boolean;
const props = defineProps({
    criteria: {
        type: Function as PropType<CriteriaFn>,
        required: true,
    },
    getOrRefresh: { type: Function as PropType<() => Promise<Array<T>>>, required: true },
    keyField: {
        type: String,
        default: undefined
    },
    minItemSize: {
        type: Number,
        required: true
    },
    scrollerHeightExpr: {
        type: String,
        required: true
    },
    placeHolder: String,
})
const emit = defineEmits(["loadResult"])

class ListItemWrapper<T> {
    inner: T;
    id: number;
    constructor(inner: T, id: number) {
        this.inner = inner;
        this.id = id;
    }
}

const search_text: Ref<string> = ref('');
const list_items: Ref<Array<ListItemWrapper<T>>> = shallowRef([]);
const displayed_items: ComputedRef<Array<ListItemWrapper<T>>> = computed(() => {
    if (search_text.value.length === 0) return list_items.value;
    return list_items.value.filter((v) => props.criteria(v.inner, search_text.value))
})
let last_id = 0;
function refresh() {
    last_id = 0;
    props.getOrRefresh().then(v => list_items.value = v.map((v) => {
        emit("loadResult", { result: true });
        last_id += 1; return new ListItemWrapper(v, last_id)
    })).catch((e) => { if (isBodyless()) { emit("loadResult", Status.NoBackend); return }; emit("loadResult", Status.Error); console.error(e) })
}
refresh()
</script>
<template>
    <SearchBar v-model="search_text" :place-holder="props.placeHolder" :refresh="refresh" />
    <DynamicScroller :items="displayed_items" :min-item-size="props.minItemSize" :buffer="200"
        :key-field="props.keyField" :style="`height: calc(${props.scrollerHeightExpr})`" class="overflow-auto">
        <template v-slot="{ item, index, active }">
            <DynamicScrollerItem :item="item" :active="active" :data-index="index">
                <slot :item="item.inner"></slot>
            </DynamicScrollerItem>
        </template>
    </DynamicScroller>
</template>