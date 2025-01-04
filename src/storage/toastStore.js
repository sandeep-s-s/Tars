import { writable } from 'svelte/store';

export const toastStore = writable({
    message: '',
    show: false,
});

export function showToast(message) {
    toastStore.set({ message, show: true });

    setTimeout(() => {
        toastStore.set({ message: '', show: false });
    }, 3000);
}
