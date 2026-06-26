<script setup lang="ts">
import { getIcon } from 'material-file-icons';
import { computed, ref, watch } from "vue";
import FolderIcon from '@mdi/svg/svg/folder.svg';
import { useElementVisibility, useTimeAgo } from "@vueuse/core";
import useViewMode from "@/composables/useViewMode.ts";
import useFileSize from "@/composables/useFileSize.ts";

const {
    item,
} = defineProps<{
    item: IndexResponseItem;
}>();

const emit = defineEmits<{
    navigate: [string],
}>();

const icon = computed(() => getIcon(item.path));
const viewMode = useViewMode();
const fileSize = useFileSize(item.size);
const isParent = item.name === '..';
const modifiedDate = computed(() => new Date(item.mtime * 1000));
const modified = useTimeAgo(modifiedDate);

const iconEl = ref<HTMLSpanElement>();
const currentVisibility = useElementVisibility(iconEl);
const visibility = ref(false);
watch(currentVisibility, currentVisibility => visibility.value ||= currentVisibility);
</script>

<template>
    <div v-if="viewMode === 'gallery'" class="wrapper gallery">
        <a
            class="file-item"
            :data-mime="item.mime_type"
            :data-filename="item.name"
            :href="`${item.path}${item.is_dir && item.path !== '/' ? '/' : ''}`"
            :target="item.is_dir ? undefined : '_blank'"
            @click="item.is_dir && ($event.preventDefault(), emit('navigate', `${item.path}${item.is_dir && item.path !== '/' ? '/' : ''}`))"
        >
            <!-- icon -->
            <span
                v-if="viewMode === 'gallery' && item.mime_type.startsWith('image/')"
                ref="iconEl"
                class="icon"
                :style="{
                    backgroundImage: visibility ? `url(/api/thumbnail?path=${encodeURIComponent(item.path)})` : undefined,
                }"
            />
            <span v-else-if="isParent" class="icon">
                <folder-icon class="size-full" />
            </span>
            <span v-else-if="item.is_dir" class="icon">
                <folder-icon class="size-full" />
            </span>
            <span v-else-if="icon" v-html="icon.svg" class="icon" />
                <!-- /icon -->
            <span class="filename">
                {{ item.name }}
            </span>
        </a>
        <div class="full-filename">{{ item.name }}</div>
    </div>
    <a
        v-else
        class="wrapper list"
        :data-mime="item.mime_type"
        :data-filename="item.name"
        :href="`${item.path}${item.is_dir && item.path !== '/' ? '/' : ''}`"
        :target="item.is_dir ? undefined : '_blank'"
        @click="item.is_dir && ($event.preventDefault(), emit('navigate', `${item.path}${item.is_dir && item.path !== '/' ? '/' : ''}`))"
    >
        <span>
            <span v-if="item.is_dir" class="icon">
                <folder-icon class="size-full" />
            </span>
            <span v-else-if="icon" v-html="icon.svg" class="icon" />
            <span class="filename">
                {{ item.name }}{{' '}}
                <span v-if="!item.is_dir" class="mime">
                    [{{ item.mime_type }}]
                </span>
            </span>
        </span>
        <span class="filesize">{{ item.is_dir ? '' : fileSize }}</span>
        <span class="mtime" :title="modified">{{ isParent ? '' : modifiedDate.toISOString().replace('.000', '') }}</span>
    </a>
</template>

<style scoped>
@reference "../style.css";

.icon > svg {
    @apply fill-gray-300;
}

.wrapper.gallery {
    @apply relative my-3;

    .file-item {
        @apply w-50 flex flex-col overflow-hidden;

        .icon {
            @apply w-50 h-50 bg-cover rounded-lg;
        }

        .filename {
            @apply overflow-hidden mt-2 max-w-full text-ellipsis text-center;

            .mime {
                @apply text-gray-400;
            }
        }
    }

    .full-filename {
        @apply absolute top-52 -mx-2 px-2 -my-1 py-1 w-54
               opacity-0 pointer-events-none transition-opacity
               rounded-sm bg-gray-300 text-gray-900
               wrap-break-word z-1 text-center;
    }

    &:hover > .full-filename {
        @apply opacity-100 pointer-events-auto;
    }
}

.wrapper.list {
    @apply contents;

    & > span {
        @apply table-cell transition-colors;

        .icon {
            @apply inline-block size-8 mr-2;
        }

        .filename {
            .mime {
                @apply text-gray-500;
            }
        }
    }

    &:nth-child(2n - 1) > span {
        @apply bg-gray-700;
    }

    &:hover > span {
        @apply bg-gray-600;
    }
}
</style>
