<script setup lang="ts">
import type { ClassValue } from "vue";

const {
    strokeWidth = 3,
    size = 24,
    class: _class,
} = defineProps<{
    strokeWidth?: number;
    size?: number;
    class?: ClassValue,
}>();


</script>

<template>
    <svg :class="_class" :width="size" :height="size" :viewBox="`0 0 24 24`" xmlns="http://www.w3.org/2000/svg">
        <g class="spinner">
            <circle cx="12" cy="12" r="9.5" fill="none" stroke="currentColor" :stroke-width="strokeWidth"/>
        </g>
    </svg>
</template>

<style scoped>
.spinner {
    transform-origin: center;
    animation: rotation 2s linear infinite;
}

.spinner circle {
    stroke-linecap: round;
    animation: stroke 1.5s ease-in-out infinite;
}

@keyframes rotation {
    100% {
        transform: rotate(360deg)
    }
}

@keyframes stroke {
    0% {
        stroke-dasharray: 0 150;
        stroke-dashoffset: 0;
    }
    47.5% {
        stroke-dasharray: 42 150;
        stroke-dashoffset: -16;
    }
    95%, 100% {
        stroke-dasharray: 42 150;
        stroke-dashoffset: -59;
    }
}
</style>