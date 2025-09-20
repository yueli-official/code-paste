<template>
  <div class="text-gray-800 dark:text-gray-200">
    <!-- 主题切换 -->
    <button
      @click="toggleDark()"
      class="relative inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9"
      aria-label="切换主题"
    >
      <!-- 太阳图标 (light 模式显示) -->
      <svg
        class="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <circle cx="12" cy="12" r="5" />
        <path
          d="m12 1v2m0 18v2M4.22 4.22l1.42 1.42m12.72 12.72l1.42 1.42M1 12h2m18 0h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"
        />
      </svg>

      <!-- 月亮图标 (dark 模式显示) -->
      <svg
        class="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>
    </button>
  </div>
</template>
<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core'
const isDark = useDark({
  initialValue: 'dark',
})
const toggleDark = useToggle(isDark)
</script>
