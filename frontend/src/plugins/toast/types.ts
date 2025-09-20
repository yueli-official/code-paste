export interface Toast {
  id: string;
  type: "info" | "success" | "warning" | "error";
  title?: string;
  message: string;
  duration?: number;
  persistent?: boolean;
}

export type ToastType = Toast["type"];
