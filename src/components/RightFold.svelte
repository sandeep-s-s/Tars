<script>
    import Request from "./Request.svelte";
    import Response from "./Response.svelte";
    export let requestUUid = "";
    export let response = {};
</script>

<div class="right-fold-container "  >
    {#if requestUUid}
        <div class="row">
            <div class="col-7" style="border-right: 1px solid #ccc;">
                <Request {requestUUid} bind:response />
            </div>
            <div class="col-5">
                <Response {response} />
            </div>
        </div>
    {/if}
</div>
