<script setup lang="ts">
import { computed } from "vue";

const location = window.location;

const { basePath, path } = defineProps<{
    basePath?: string;
    path: string;
}>();

const emit = defineEmits<{
    navigate: [string],
}>();

const pathComponents = computed(() => {
    const components = path
        .replace(/^\//, '')
        .split('/')
        .filter(c => !!c);
    return components.map((component, i) => ({
        name: component,
        path: `/${components.slice(0, i + 1).join('/')}/`,
    }));
});

const root = computed(() => {
    let str = basePath || location.host;
    // if (str !== '/') {
    //     str += '/';
    // }
    return str;
})
</script>

<template>
    <h1 class="mb-4 grow">
        <a
            href="/"
            @click.prevent="emit('navigate', '/')"
            class="prefix"
        >
            {{ root }}
        </a>
        <span class="slash">/</span>
        <template v-for="component in pathComponents">
            <a
                :href="component.path"
                class="component"
                @click.prevent="emit('navigate', component.path)"
            >
                {{ component.name }}
            </a>
            <span class="slash">/</span>
        </template>
    </h1>
</template>

<style scoped>
@reference "../style.css";

.prefix {
    @apply text-gray-400 text-lg
    hover:underline mr-2;
}

.slash, .component {
    @apply text-3xl;

    &.slash {
        @apply text-gray-500 mx-2;
    }

    &.component {
        @apply text-gray-200 hover:underline;
    }
}
</style>