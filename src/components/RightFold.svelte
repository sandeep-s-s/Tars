<script>
    import Request from "./Request.svelte";
    import Response from "./Response.svelte";
    export let requestUUid = "";
    let response = {}
</script>

<div class="right-fold-container">
    <Request {requestUUid} bind:response={response}/>
    <Response  {response}/>
</div>

<style>
    .right-fold-container {
        display: flex;
        margin: 10px;
    }
</style>
