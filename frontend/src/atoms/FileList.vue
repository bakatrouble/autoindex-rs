<script setup lang="ts">
import FileItemParent from "@/atoms/FileItemParent.vue";
import FileItem from "@/atoms/FileItem.vue";
import useViewMode from "@/composables/useViewMode.ts";
import alphaSort from "alpha-sort";
import { computed } from "vue";

const {
    index,
} = defineProps<{
    path: string;
    index: IndexResponse;
}>();

const sortedItems = computed(() => index.items.toSorted((a, b) => {
    if (a.is_dir === b.is_dir) {
        return alphaSort({ natural: true })(a.name, b.name);
    }
    return a.is_dir ? -1 : 1;
}));

const emit = defineEmits<{
    navigate: [string],
}>();

const viewMode = useViewMode();
</script>

<template>
    <div :class="['list-wrapper', viewMode]">
        <div v-if="viewMode === 'list'" class="wrapper list header">
            <div>Filename</div>
            <div>Size</div>
            <div>Modified</div>
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