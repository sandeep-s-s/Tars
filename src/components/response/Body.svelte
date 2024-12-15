<script>
    export let response = {};
    import httpStatusCodes from '../../data/httpStatusCodes.json';
    
</script>

<div class="response-body">
    {#if response.message}
        <span>Status : {response.status_code} : {httpStatusCodes[response.status_code]} </span>
        <pre>  {response.message} </pre>
    {/if}
</div>

<style>
    .response-body{
        margin-top: 10px;
    }
</style>
