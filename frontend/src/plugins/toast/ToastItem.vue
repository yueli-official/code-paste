<template>
  <Transition
    enter-active-class="transition-all duration-300 ease-out"
    enter-from-class="opacity-0 transform translate-x-full"
    enter-to-class="opacity-100 transform translate-x-0"
    leave-active-class="transition-all duration-200 ease-in"
    leave-from-class="opacity-100 transform translate-x-0"
    leave-to-class="opacity-0 transform translate-x-full"
  >
    <div
      v-if="visible"
      :class="toastClasses"
      class="relative flex items-start gap-3 p-4 mb-3 rounded-lg shadow-md border min-w-80 max-w-md"
      role="alert"
    >
      <!-- 图标 -->
      <div class="flex-shrink-0 mt-0.5">
        <component :is="iconComponent" :class="iconClasses" class="w-5 h-5" />
      </div>

      <!-- 内容 -->
      <div class="flex-1 min-w-0">
        <h4
          v-if="toast.title"
          :class="titleClasses"
          class="text-sm font-semibold mb-1"
        >
          {{ toast.title }}
        </h4>
        <p :class="messageClasses" class="text-sm">
          {{ toast.message }}
        </p>
      </div>

      <!-- 关闭按钮 -->
      <button
        @click="handleClose"
        :class="closeButtonClasses"
        class="flex-shrink-0 p-1 rounded-md hover:bg-opacity-20 transition-colors"
        aria-label="关闭"
      >
        <XIcon class="w-4 h-4" />
      </button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, h } from "vue";
import type { Toast } from "./types";

// 图标组件
const InfoIcon = () =>
  h("svg", { viewBox: "0 0 24 24", fill: "currentColor" }, [
    h("path", {
      d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-6h2v6zm0-8h-2V9h2v2z",
    }),
  ]);

const CheckCircleIcon = () =>
  h("svg", { viewBox: "0 0 24 24", fill: "currentColor" }, [
    h("path", {
      d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
    }),
  ]);

const ExclamationTriangleIcon = () =>
  h("svg", { viewBox: "0 0 24 24", fill: "currentColor" }, [
    h("path", { d: "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z" }),
  ]);

const XCircleIcon = () =>
  h("svg", { viewBox: "0 0 24 24", fill: "currentColor" }, [
    h("path", {
      d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z",
    }),
  ]);

const XIcon = () =>
  h(
    "svg",
    {
      viewBox: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      "stroke-width": "2",
    },
    [h("path", { d: "M18 6L6 18M6 6l12 12" })]
  );

interface Props {
  toast: Toast;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  close: [id: string];
}>();

const visible = ref(false);

// 图标映射
const iconMap = {
  info: InfoIcon,
  success: CheckCircleIcon,
  warning: ExclamationTriangleIcon,
  error: XCircleIcon,
};

const iconComponent = computed(() => iconMap[props.toast.type]);

// 样式类映射

// 样式类映射
const styleMap = {
  info: {
    container:
      "bg-[var(--color-card)] border-[var(--color-border)] text-[var(--color-card-foreground)] border-l-4 border-l-primary/50 dark:border-l-primary/40",
    icon: "text-[var(--color-primary)]",
    title: "text-[var(--color-card-foreground)]",
    message: "text-[var(--color-muted-foreground)]",
    close:
      "text-[var(--color-muted-foreground)] hover:text-[var(--color-card-foreground)] hover:bg-[var(--color-muted)]",
  },
  success: {
    container:
      "bg-[var(--color-card)] border-[var(--color-border)] text-[var(--color-card-foreground)] border-l-4 border-l-green-500 dark:border-l-green-400",
    icon: "text-green-600 dark:text-green-400",
    title: "text-[var(--color-card-foreground)]",
    message: "text-[var(--color-muted-foreground)]",
    close:
      "text-[var(--color-muted-foreground)] hover:text-[var(--color-card-foreground)] hover:bg-[var(--color-accent)]",
  },
  warning: {
    container:
      "bg-[var(--color-card)] border-[var(--color-border)] text-[var(--color-card-foreground)] border-l-4 border-l-amber-500 dark:border-l-amber-400",
    icon: "text-amber-600 dark:text-amber-400",
    title: "text-[var(--color-card-foreground)]",
    message: "text-[var(--color-muted-foreground)]",
    close:
      "text-[var(--color-muted-foreground)] hover:text-[var(--color-card-foreground)] hover:bg-[var(--color-accent)]",
  },
  error: {
    container:
      "bg-[var(--color-card)] border-[var(--color-border)] text-[var(--color-card-foreground)] border-l-4 border-l-red-500 dark:border-l-red-400",
    icon: "text-red-600 dark:text-red-400",
    title: "text-[var(--color-card-foreground)]",
    message: "text-[var(--color-muted-foreground)]",
    close:
      "text-[var(--color-muted-foreground)] hover:text-[var(--color-card-foreground)] hover:bg-[var(--color-accent)]",
  },
};

const toastClasses = computed(() => styleMap[props.toast.type].container);
const iconClasses = computed(() => styleMap[props.toast.type].icon);
const titleClasses = computed(() => styleMap[props.toast.type].title);
const messageClasses = computed(() => styleMap[props.toast.type].message);
const closeButtonClasses = computed(() => styleMap[props.toast.type].close);

const handleClose = () => {
  visible.value = false;
  setTimeout(() => {
    emit("close", props.toast.id);
  }, 500);
};

onMounted(() => {
  visible.value = true;
});
</script>
