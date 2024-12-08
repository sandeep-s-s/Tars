<script>
    export let response = {};
</script>

<div class="response-container">
    {#if response}
        <pre>{response.message}</pre>
        <pre>{JSON.stringify(response, null, 2)}</pre>
    {/if}
</div>
