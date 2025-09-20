import { useToast } from "./useToast";
import type { Toast } from "./types";

const { addToast } = useToast();

interface ToastOptions {
  title?: string;
  duration?: number;
  persistent?: boolean;
}

const createToastMethod = (type: Toast["type"]) => {
  return (message: string, options?: ToastOptions): string => {
    return addToast({
      type,
      message,
      ...options,
    });
  };
};

export const toast = {
  info: createToastMethod("info"),
  success: createToastMethod("success"),
  warning: createToastMethod("warning"),
  error: createToastMethod("error"),

  // 自定义方法
  custom: (toast: Omit<Toast, "id">): string => {
    return addToast(toast);
  },
};
