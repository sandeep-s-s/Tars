<script>
    export let request = {};
</script>

<div class="request-form-data-container">
    {#if request && request["body"] && request["body"].xWwwFormUrlencoded}
        {#each request["body"].xWwwFormUrlencoded as v, i}
            <div class="form-data-container">
                <div class="col-4">
                    <input
                        id={i}
                        type="text"
                        bind:value={v.key}
                        placeholder="key"
                        class="form-control form-control-sm"
                    />
                </div>
                <div class="col-4">
                    <input
                        id={i}
                        type="text"
                        bind:value={v.value}
                        placeholder="value"
                        class="form-control form-control-sm"
                    />
                </div>
                <div class="col-1">
                    <input
                        type="checkbox"
                        bind:checked={v.checked}
                        on:change={(e) => addChecked(i)}
                        title="Select"
                    />
                </div>
                <div class="col-2">
                    <button
                        on:click={(e) => deleteElement(i)}
                        class="btn btn-outline-danger"
                        title="Delete">Delete</button
                    >
                </div>
            </div>
        {/each}
    {/if}
</div>


<style>
    .form-data-container {
        margin: 5px;
        display: flex;
        gap: 15px;
    }
</style>
