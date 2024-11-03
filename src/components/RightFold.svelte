<script>
    import Request from "./Request.svelte";
    export let requestUUid = "";
    console.log(requestUUid)
</script>

<div class="right-fold-container">
    <Request {requestUUid} />
</div>

<style>
    .right-fold-container {
        display: flex;
        margin: 10px;
    }
</style>
