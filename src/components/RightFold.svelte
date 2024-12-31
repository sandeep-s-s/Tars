<script>
    import Request from "./Request.svelte";
    import Response from "./Response.svelte";
    export let requestUUid = "";
    export let response = {};
</script>

<div class="right-fold-container">
    {#if requestUUid}
        <div class="row mt-5">
            <div class="col-8">
                <Request {requestUUid} bind:response />
            </div>
            <div class="col-4">
                <Response {response} />
            </div>
        </div>
    {/if}
</div>
