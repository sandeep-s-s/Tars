<script>
    import { toastStore } from "../storage/toastStore";
    import { onDestroy } from "svelte";

    let toast = { message: "", show: false };

    const unsubscribe = toastStore.subscribe((value) => {
        toast = value;
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<div class="toast-container position-fixed bottom-0 end-0 p-3">
    <div
        class="toast align-items-center text-white bg-dark border-0"
        class:show={toast.show}
        role="alert"
        aria-live="assertive"
        aria-atomic="true"
    >
        <div class="d-flex">
            <div class="toast-body">
                {toast.message}
            </div>
            <button
                type="button"
                class="btn-close btn-close-white me-2 m-auto"
                on:click={() => toastStore.set({ message: "", show: false })}
                aria-label="Close"
            ></button>
        </div>
    </div>
</div>

<style>
    .toast {
        transition: opacity 0.5s ease-in-out;
    }
    .toast.show {
        opacity: 1;
    }
    .toast:not(.show) {
        opacity: 0;
    }
</style>
