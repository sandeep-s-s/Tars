<script>
    export let response = {};
</script>

<div class="response-header">
    {#if response.headers}
        <table>
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
    table {
        width: 100%;
        border-collapse: collapse;
    }
    th,
    td {
        border: 1px solid #ddd;
        padding: 8px;
        text-align: left;
    }
    th {
        background-color: #f2f2f2;
    }

    .response-header{
        margin-top: 10px;
    }
</style>
