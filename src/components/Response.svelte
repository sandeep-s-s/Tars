<script>
    import ResponseNav from "./response/ResponseNav.svelte";


    export let response = {};
</script>

<!-- <div class="response-container">
    <h3>Response</h3>
    {#if response.message}
        <span>Status : {response.status_code}</span>
        <pre>  {response.message} </pre>
    {/if}
</div> -->

<style>
</style>

<ResponseNav {response}/>