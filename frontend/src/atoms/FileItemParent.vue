<script setup lang="ts">
import Path from 'path-browserify-esm';
import { computed } from "vue";
import FileItem from "@/atoms/FileItem.vue";

const { path } = defineProps<{
    path: string;
}>();
const emit = defineEmits<{
    navigate: [string];
}>();

const parentPath = computed(() => {
    const parent = Path.dirname(path);
    if (parent === '.') {
        return '/';
    }
    return parent;
})
</script>

<template>
    <file-item
        v-if="path !== '/'"
        :item="{
            name: '..',
            path: parentPath,
            size: 0,
            mime_type: 'application/octet-stream',
            mtime: 0,
            is_dir: true
        }"
        @navigate="emit('navigate', $event)"
    />
</template>

<style scoped>

</style>