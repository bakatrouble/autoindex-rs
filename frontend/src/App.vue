<script setup lang="ts">
import { onMounted, provide, ref, watch } from 'vue';
import { useQuery, useQueryClient } from "@tanstack/vue-query";
import { client } from "@/utils.ts";
import Breadcrumbs from "@/atoms/Breadcrumbs.vue";
import LoadingSpinner from "@/atoms/LoadingSpinner.vue";
import { useNProgress } from "@vueuse/integrations/useNProgress";
import 'nprogress/nprogress.css';
import { HTTPError } from "ky";
import FileList from "@/atoms/FileList.vue";
import { useLocalStorage } from "@vueuse/core";

const config = ref(window.initialConfig || {});
const queryClient = useQueryClient();

const path = ref(decodeURIComponent(location.pathname));
const { isLoading } = useNProgress();
const viewMode = useLocalStorage<ViewMode>('viewMode', config.value.default_mode || 'gallery');
provide('viewMode', viewMode);

watch(path, path => {
    const sanitizedPath = path.replace('%', '%25').replace('#', '%23').replace('?', '%3F');
    history.pushState({}, '', sanitizedPath);
});
watch(path, path => {
    document.title = `autoindex: ${path}`;
}, { immediate: true });
onMounted(() => {
    addEventListener('popstate', () => path.value = decodeURIComponent(location.pathname));

    const es = new EventSource("/api/events");
    es.addEventListener('message', async e => {
        if (e.data === 'update') {
            config.value = await client.get('/config?json=true').json();
            path.value = '/';
            await queryClient.invalidateQueries({
                queryKey: ['index'],
            });
        }
    });
});

const {
    data: index,
    error: indexError,
    isError: indexIsError,
    isPending: indexIsPending,
    isLoading: indexIsLoading,
} = useQuery({
    queryKey: ['index', path],
    queryFn: async () => {
        const r = await client.get(
            `/index`,
            { searchParams: { path: path.value } }
        ).json();
        return r as IndexResponse;
    },
    retry: (failureCount, error) => {
        if (error instanceof HTTPError && error.response.status === 404) {
            return false;
        }
        return failureCount < 3;
    },
});
watch(
    indexIsLoading,
    indexIsLoading => isLoading.value = indexIsLoading,
    { immediate: true },
);
</script>

<template>
    <div class="flex items-center container mx-auto">
        <breadcrumbs
            :base-path="config.root"
            :path="path"
            @navigate="path = $event"
        />
        <span
            class="cursor-pointer hover:underline whitespace-nowrap"
            @click="viewMode = viewMode == 'list' ? 'gallery' : 'list'"
        >
            [{{ viewMode === 'gallery' ? 'List' : 'Gallery' }} mode]
        </span>
    </div>
    <div
        v-if="indexIsPending"
        class="grow flex flex-col items-center justify-center"
    >
        <loading-spinner class="mb-2" :size="100" :stroke-width="2" />
        <span>Loading</span>
    </div>
    <div v-else-if="indexIsError" class="grow flex flex-col items-center justify-center">
        <template
            v-if="indexError instanceof HTTPError && indexError.response"
        >
            <span class="text-[100px]/[100px]">{{ indexError.response.status }}</span>
            <span>{{ indexError.response.statusText }}</span>
        </template>
        <template v-else>
            <span class="text-[100px]/[100px]">Error</span>
            <span>An unexpected error has occurred</span>
        </template>
    </div>
    <template v-else-if="index">
        <file-list
            :path
            :index
            @navigate="path = $event"
        />
    </template>
</template>

<style scoped>
@reference "./style.css";


</style>
