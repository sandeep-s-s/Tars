<script>
    import Request from "./Request.svelte";
    import Response from "./Response.svelte";
    export let requestUUid = "";
    export let response = {};
</script>

<div class="right-fold-container">
    {#if requestUUid}
        <Request {requestUUid} bind:response />
        <div class="response">
            <Response {response} />
        </div>
    {/if}
</div>

<style>
    .right-fold-container {
        display: flex;
        margin: 10px;
    }
    .response {
        display: flex;
        border-left: 1px solid whitesmoke;
        padding: 10px;
        margin: 10px;
    }
</style>
