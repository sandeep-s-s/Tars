<script>
    export let response = {};
</script>

<div class="response-container">
    {#if response.message}
        <pre>{response.message}</pre>
    {/if}
</div>
