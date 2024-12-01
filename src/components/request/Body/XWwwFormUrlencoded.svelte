<script>
    export let request = {};

    if (
        request &&
        request["body"].xWwwFormUrlencoded &&
        request["body"].xWwwFormUrlencoded.length === 0
    ) {
        request["body"].xWwwFormUrlencoded.push({
            key: "",
            value: "",
            checked: false,
        });
    }
    const addField = () => {
        const xWwwFormUrlencoded = request["body"].xWwwFormUrlencoded;

        const newObject = {
            key: "",
            value: "",
            type: "text",
            checked: false,
        };

        const updatedFormData = [...xWwwFormUrlencoded, newObject];

        request["body"].xWwwFormUrlencoded = updatedFormData;
    };

    const removeField = () => {
        request["body"].xWwwFormUrlencoded = request["body"].xWwwFormUrlencoded.slice(
            0,
            request["body"].xWwwFormUrlencoded.length - 1,
        );
    };

    const addChecked = (i) => {
        request["body"].xWwwFormUrlencoded[i].checked =
            request["body"].xWwwFormUrlencoded[i].checked;
    };
    const deleteElement = (index) => {
        if (index > -1) {
            request["body"].xWwwFormUrlencoded.splice(index, 1); 
        }
        request["body"].xWwwFormUrlencoded = request["body"].xWwwFormUrlencoded;
    };
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
<button
    on:click|preventDefault={addField}
    class="btn btn-outline-danger"
    title="Add New">Add</button
>


<style>
    .form-data-container {
        margin: 5px;
        display: flex;
        gap: 15px;
    }
</style>
