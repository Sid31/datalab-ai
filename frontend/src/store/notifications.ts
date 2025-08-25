import { writable } from 'svelte/store';

export interface Notification {
  type: 'error' | 'info' | 'success';
  message: string;
  id: number;
}

export type NewNotification = Omit<Notification, 'id'>;

let nextId = 0;

export const notifications = writable<Notification[]>([]);

export function addNotification(notification: NewNotification, timeout = 2000) {
  const id = nextId++;

  notifications.update(($n) => [...$n, { ...notification, id }]);

  setTimeout(() => {
    notifications.update(($n) => $n.filter((n) => n.id != id));
  }, timeout);
}

export function showError(message: string) {
  addNotification({ type: 'error', message });
}

export function showSuccess(message: string) {
  addNotification({ type: 'success', message });
}

export function showInfo(message: string) {
  addNotification({ type: 'info', message });
}
