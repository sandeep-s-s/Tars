<script>
    export let response = {};
</script>

<div class="response-header">
    {#if response.headers}
        <table class="table table-striped">
            <thead>
                <tr>
                    <th>Header</th>
                    <th>Value</th>
                </tr>
            </thead>
            <tbody>
                {#each Object.entries(response.headers) as [key, value]}
                    <tr>
                        <td>{key}</td>
                        <td>{value}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</div>

<style>
  
</style>
