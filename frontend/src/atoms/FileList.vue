<script setup lang="ts">
import FileItemParent from "@/atoms/FileItemParent.vue";
import FileItem from "@/atoms/FileItem.vue";
import useViewMode from "@/composables/useViewMode.ts";
import alphaSort from "alpha-sort";
import { computed } from "vue";
import { useLocalStorage } from "@vueuse/core";
import ChevronUpIcon from '@mdi/svg/svg/chevron-up.svg';
import ChevronDownIcon from '@mdi/svg/svg/chevron-down.svg';

const viewMode = useViewMode();

const {
    index,
} = defineProps<{
    path: string;
    index: IndexResponse;
}>();

type SortingField = 'name' | 'size' | 'modified';
type SortingValue = `${'-' | ''}${SortingField}`;
const sorting = useLocalStorage<SortingValue>('sorting', 'name');
const actualSorting = computed(() => viewMode === 'gallery' ? 'name' : sorting.value);

const sortedItems = computed(() => index.items.toSorted((a, b) => {
    const nameSorter = (a: IndexResponseItem, b: IndexResponseItem) => {
        if (a.is_dir === b.is_dir) {
            return alphaSort({ natural: true, caseInsensitive: true })(a.name, b.name);
        }
        return a.is_dir ? -1 : 1;
    }
    const sizeSorter = (a: IndexResponseItem, b: IndexResponseItem) => {
        return a.size > b.size ? 1 : -1;
    }

    const modifiedSorter = (a: IndexResponseItem, b: IndexResponseItem) => {
        return a.mtime > b.mtime ? 1 : -1;
    }

    let sorter: (a: IndexResponseItem, b: IndexResponseItem) => number;
    switch (actualSorting.value) {
        case 'name':
        case '-name':
            sorter = nameSorter;
            break;
        case 'size':
        case '-size':
            sorter = sizeSorter;
            break
        case 'modified':
        case '-modified':
            sorter = modifiedSorter;
            break;
    }

    const result = sorter(a, b);
    return actualSorting.value.startsWith('-') ? -result : result;
}));

const emit = defineEmits<{
    navigate: [string],
}>();

const toggleSort = (field: SortingField) => {
    if (sorting.value.includes(field)) {
        if (sorting.value.startsWith('-')) {
            sorting.value = sorting.value.slice(1) as SortingValue;
        } else {
            sorting.value = `-${sorting.value}` as SortingValue;
        }
    } else {
        sorting.value = field;
    }
}
</script>

<template>
    <div :class="['list-wrapper', viewMode]">
        <div v-if="viewMode === 'list'" class="wrapper list header">
            <span @click="toggleSort('name')">
                Filename{{' '}}
                <template v-if="actualSorting.includes('name')">
                    <chevron-down-icon v-if="!actualSorting.startsWith('-')" />
                    <chevron-up-icon v-else />
                </template>
            </span>
            <span @click="toggleSort('size')">
                Size{{' '}}
                <template v-if="actualSorting.includes('size')">
                    <chevron-down-icon v-if="!actualSorting.startsWith('-')" />
                    <chevron-up-icon v-else />
                </template>
            </span>
            <span @click="toggleSort('modified')">
                Modified{{' '}}
                <template v-if="actualSorting.includes('modified')">
                    <chevron-down-icon v-if="!actualSorting.startsWith('-')" />
                    <chevron-up-icon v-else />
                </template>
            </span>
        </div>
        <file-item-parent
            v-if="path !== '/'"
            :path="path"
            @navigate="emit('navigate', $event)"
            key=".."
        />
        <file-item
            v-for="item in sortedItems"
            :item
            @navigate="emit('navigate', $event)"
            :key="item.path"
        />
        <div v-if="viewMode === 'gallery'" v-for="_ in Array(20)" class="gap" />
    </div>
</template>

<style scoped>
@reference "../style.css";

.list-wrapper.gallery {
    @apply justify-center
           -mx-5;

    &.gallery {
        @apply grid grid-cols-[repeat(auto-fit,--spacing(60))]
               gap-x-5;
    }
}

.list-wrapper.list {
    @apply grid container mx-auto border border-gray-600 rounded-lg;

    grid-template-columns:
        1fr
        minmax(--spacing(30), max-content)
        minmax(--spacing(45), max-content);

    .header {
        @apply contents;

        & > * {
            @apply cursor-pointer;

            svg {
                @apply fill-gray-300 size-6;
            }

            &:hover {
                @apply bg-gray-600;
            }
        }
    }

    :global(.wrapper.list > *) {
        @apply px-2 py-1 h-full flex items-center border-r border-b border-gray-600;
    }

    :global(.wrapper.list:last-child > *) {
        @apply border-b-0;
    }

    :global(.wrapper.list > *:last-child) {
        @apply border-r-0;
    }

    .header > *:last-child {
        @apply rounded-tr-lg;
    }

    :global(.wrapper.list:last-child > *:first-child) {
        @apply rounded-bl-lg;
    }

    :global(.wrapper.list:last-child > *:last-child) {
        @apply rounded-br-lg;
    }
}
</style>