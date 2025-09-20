<template>
  <Teleport to="body">
    <div
      class="fixed top-4 right-4 z-50 pointer-events-none"
      role="region"
      aria-label="通知"
      aria-live="polite"
    >
      <div class="flex flex-col">
        <ToastItem
          v-for="toast in toasts"
          :key="toast.id"
          :toast="toast"
          @close="removeToast"
          class="pointer-events-auto"
        />
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { useToast } from './useToast'
import ToastItem from './ToastItem.vue'
const { toasts, removeToast } = useToast()
</script>
