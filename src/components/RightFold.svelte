<script>
    import { onMount, onDestroy } from "svelte";
    import Request from "./Request.svelte";
    import Response from "./Response.svelte";
    import { invoke } from "@tauri-apps/api/core";

    export let requestUUid;
    export let response = {};

    let tabs = [];
    let draggedIndex = null;
    let dragOverIndex = null;
    let startX = 0;
    let startY = 0;
    let isDragging = false;
    const dragThreshold = 5;

    async function getTabs() {
        tabs = await invoke("get_tabs");
        tabs = tabs.sort((a, b) => a.order_id - b.order_id);
    }
    
    onMount(async () => {
        await getTabs();
        if (!requestUUid && tabs.length > 0) {
            requestUUid = tabs.find((tab) => tab.is_active)?.request_uuid || null;
        }
    });
    
    function handleTabsRefresh() {
        getTabs();
    }
    
    onMount(() => {
        window.addEventListener('tabsRefresh', handleTabsRefresh);
        return () => {
            window.removeEventListener('tabsRefresh', handleTabsRefresh);
        };
    });

    let previousRequestUUid = null;
    
    $: if (requestUUid && requestUUid !== previousRequestUUid) {
        previousRequestUUid = requestUUid;
        updateTabState(requestUUid);
    }

    async function updateTabState(uuid) {
        const tabExists = tabs.some(tab => tab.request_uuid === uuid);
        
        if (!tabExists) {
            await getTabs();
        }
        
        tabs = tabs.map((tab) => {
            if (tab.request_uuid === uuid) {
                return { ...tab, is_active: true };
            }
            return { ...tab, is_active: false };
        });
        invoke("update_tabs", { uuid });
    }

    function updateRequestUUid(uuid) {
        requestUUid = uuid;
    }

    async function closeTab(event, id) {
        event.stopPropagation();
        
        const tabToClose = tabs.find(tab => tab.request_id === id);
        const wasActive = tabToClose?.is_active;
        
        await invoke("close_tab", { id });
        await getTabs();
        
        if (wasActive) {
            if (tabs.length > 0) {
                const nextActiveTab = tabs.find(tab => tab.request_uuid !== tabToClose.request_uuid) || tabs[0];
                requestUUid = nextActiveTab?.request_uuid || null;
            } else {
                requestUUid = null;
            }
        }
    }

    function handlePointerDown(event, index) {
        if (event.button !== 0) return;
        startX = event.clientX;
        startY = event.clientY;
        draggedIndex = index;
        isDragging = false;
    }

    function handlePointerMove(event) {
        if (draggedIndex === null) return;
        
        const deltaX = event.clientX - startX;
        const deltaY = event.clientY - startY;
        
        if (!isDragging && (Math.abs(deltaX) > dragThreshold || Math.abs(deltaY) > dragThreshold)) {
            isDragging = true;
        }
        
        if (isDragging) {
            const tabElements = document.querySelectorAll('.tab-item');
            let newDragOverIndex = draggedIndex;
            
            tabElements.forEach((el, i) => {
                const rect = el.getBoundingClientRect();
                if (event.clientY >= rect.top && event.clientY <= rect.bottom) {
                    if (event.clientX < rect.left + rect.width / 2) {
                        newDragOverIndex = i;
                    } else {
                        newDragOverIndex = i + 1;
                    }
                }
            });
            
            newDragOverIndex = Math.min(Math.max(newDragOverIndex, 0), tabs.length);
            if (newDragOverIndex > draggedIndex) {
                newDragOverIndex--;
            }
            dragOverIndex = newDragOverIndex;
        }
    }

    async function handlePointerUp() {
        if (draggedIndex !== null && dragOverIndex !== null && draggedIndex !== dragOverIndex && isDragging) {
            const newTabs = [...tabs];
            const [removed] = newTabs.splice(draggedIndex, 1);
            newTabs.splice(dragOverIndex, 0, removed);

            const tabOrders = newTabs.map((tab, i) => [tab.id, i]);
            await invoke("reorder_tabs", { tabOrders });

            tabs = newTabs;
        }
        
        draggedIndex = null;
        dragOverIndex = null;
        isDragging = false;
    }

    function handleClick(event, uuid) {
        if (!isDragging) {
            updateRequestUUid(uuid);
        }
    }
</script>

<svelte:window 
    on:pointermove={handlePointerMove}
    on:pointerup={handlePointerUp}
/>

<div class="right-fold-container mt-3">
    <div class="tabs-container">
        <div class="tabs-list" role="tablist">
            {#each tabs as tab, index (tab.id)}
                <div
                    class="tab-item"
                    class:active={tab.is_active}
                    class:drag-over={dragOverIndex === index}
                    class:is-dragging={draggedIndex === index}
                    on:click={(e) => handleClick(e, tab.request_uuid)}
                    on:pointerdown={(e) => handlePointerDown(e, index)}
                    role="tab"
                    aria-selected={tab.is_active}
                >
                    <span class="tab-title">{tab.request_name}</span>
                    <button
                        class="tab-close"
                        on:click={(e) => closeTab(e, tab.request_id)}
                        aria-label="Close tab"
                    >
                        ×
                    </button>
                </div>
            {/each}
        </div>
    </div>
    {#if requestUUid}
        <div class="row">
            <div class="col-7" style="border-right: 1px solid #ccc;">
                <Request {requestUUid} bind:tabs bind:response />
            </div>
            <div class="col-5">
                <Response {response} />
            </div>
        </div>
    {/if}
</div>

<style>
    .tabs-container {
        border-bottom: 1px solid #dee2e6;
        background-color: #f8f9fa;
    }

    .tabs-list {
        display: flex;
        flex-wrap: wrap;
        padding: 0;
        margin: 0;
        list-style: none;
        gap: 0;
        overflow: hidden;
    }

    .tab-item {
        display: flex;
        align-items: center;
        padding: 8px 12px;
        cursor: grab;
        border: 1px solid transparent;
        border-bottom: none;
        background-color: #e9ecef;
        margin-right: 2px;
        border-top-left-radius: 4px;
        border-top-right-radius: 4px;
        transition: all 0.2s ease;
        user-select: none;
        font-size: 14px;
        touch-action: none;
        position: relative;
        animation: slideIn 0.3s ease;
    }

    .tab-item:hover {
        background-color: #dee2e6;
        transform: translateY(-2px);
    }

    .tab-item.active {
        background-color: #fff;
        border-color: #dee2e6;
        border-bottom-color: #fff;
    }

    .tab-item.is-dragging {
        opacity: 0.9;
        transform: scale(1.08) translateY(-6px);
        box-shadow: 0 12px 28px rgba(0, 0, 0, 0.3);
        z-index: 1000;
        cursor: grabbing;
        transition: transform 0.15s ease, box-shadow 0.15s ease;
    }

    .tab-item.drag-over:not(.is-dragging) {
        border-left: 3px solid #007bff;
        background-color: #cfe2ff;
        transform: translateX(6px);
        animation: tabPulse 0.5s ease-in-out infinite;
    }

    @keyframes tabPulse {
        0%, 100% {
            background-color: #cfe2ff;
        }
        50% {
            background-color: #b8daff;
        }
    }

    @keyframes slideIn {
        from {
            opacity: 0;
            transform: translateY(-10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .tab-title {
        max-width: 150px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        margin-right: 8px;
        pointer-events: none;
    }

    .tab-close {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 18px;
        height: 18px;
        padding: 0;
        border: none;
        background: transparent;
        border-radius: 50%;
        font-size: 16px;
        line-height: 1;
        color: #6c757d;
        cursor: pointer;
        transition: all 0.2s ease;
        transform: scale(0.8);
    }

    .tab-close:hover {
        background-color: #dc3545;
        color: #fff;
        transform: scale(1.15);
    }
</style>