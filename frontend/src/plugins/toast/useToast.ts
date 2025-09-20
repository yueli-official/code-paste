import { ref } from "vue";
import type { Toast, ToastType } from "./types";

const toasts = ref<Toast[]>([]);
let toastId = 0;

const defaultDurations: Record<ToastType, number> = {
  info: 4000,
  success: 3000,
  warning: 5000,
  error: 6000,
};

export const useToast = () => {
  const addToast = (toast: Omit<Toast, "id">): string => {
    const id = `toast-${++toastId}`;
    const newToast: Toast = {
      id,
      duration: defaultDurations[toast.type],
      ...toast,
    };

    toasts.value.push(newToast);

    if (!newToast.persistent && newToast.duration && newToast.duration > 0) {
      setTimeout(() => {
        removeToast(id);
      }, newToast.duration);
    }

    return id;
  };

  const removeToast = (id: string) => {
    const index = toasts.value.findIndex((toast) => toast.id === id);
    if (index > -1) {
      toasts.value.splice(index, 1);
    }
  };

  const clearAll = () => {
    toasts.value = [];
  };

  return {
    toasts: toasts,
    addToast,
    removeToast,
    clearAll,
  };
};
