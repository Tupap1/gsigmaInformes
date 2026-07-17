export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
  duration: number;
}

class ToastStore {
  list = $state<Toast[]>([]);

  show(message: string, type: 'success' | 'error' | 'info' = 'success', duration = 4000) {
    const id = Math.random().toString(36).substring(2);
    const toast: Toast = { id, message, type, duration };
    this.list = [...this.list, toast];
    
    if (duration > 0) {
      setTimeout(() => {
        this.dismiss(id);
      }, duration);
    }
  }

  success(message: string, duration = 4000) {
    this.show(message, 'success', duration);
  }

  error(message: string, duration = 5000) {
    this.show(message, 'error', duration);
  }

  info(message: string, duration = 4000) {
    this.show(message, 'info', duration);
  }

  dismiss(id: string) {
    this.list = this.list.filter(t => t.id !== id);
  }
}

export const toasts = new ToastStore();
