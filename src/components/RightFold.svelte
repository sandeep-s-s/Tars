<script>
    import { onMount } from "svelte";
    import Request from "./Request.svelte";
    import Response from "./Response.svelte";
    import { invoke } from "@tauri-apps/api/core";

    /**
     * @type {string | any[]}
     */
    export let requestUUid;

    export let response = {};

    /**
     * @type {any[]}
     */
    let tabs = [];

    async function getTabs() {
        tabs = await invoke("get_tabs");
        if (tabs.length == 0) {
            requestUUid = null;
        }
    }
    onMount(async () => {
        await getTabs();
        requestUUid = tabs.find((tab) => tab.is_active)?.request_uuid || null;
    });

    /**
     * @param {string} uuid
     */
    function updateRequestUUid(uuid) {
        requestUUid = uuid;
    }

    /**
     * @param {string} id
     */
    async function closeTab(id) {
        await invoke("close_tab", { id });
        getTabs();
    }
</script>

<div class="right-fold-container mt-3">
    <div class="btn-group" role="group" aria-label="Dynamic Button Group">
        {#each tabs as tab}
            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a
                        class="nav-link {tab.is_active ? 'active' : ''}"
                        id="home-tab"
                        data-toggle="tab"
                        href="#home"
                        role="tab"
                        aria-controls="home"
                        aria-selected="true"
                        on:click={() => updateRequestUUid(tab.request_uuid)}
                    >
                        {tab.request_name}
                    </a>
                    <i
                        class="bi bi-x-circle-fill"
                        on:click={() => closeTab(tab.request_id)}
                        >
                    </i>
                </li>
            </ul>
        {/each}
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
    .nav-item {
        display: flex;
        align-items: center;
    }
    .bi {
        margin-right: 8px;
        display: flex;
        align-items: center;
    }
</style>
