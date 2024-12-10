<script>
    import FormDataFile from "./FormDataFile.svelte";

    export let request = {};

    if (
        request &&
        request["body"].formData &&
        request["body"].formData.length === 0
    ) {
        request["body"].formData.push({
            key: "",
            value: "",
            checked: false,
            type: "",
            src: "",
        });
    }
    const addField = () => {
        const formData = request["body"].formData;

        const newObject = {
            key: "",
            value: "",
            type: "",
            checked: false,
            src: "",
        };

        const updatedFormData = [...formData, newObject];

        request["body"].formData = updatedFormData;
    };

    const removeField = () => {
        request["body"].formData = request["body"].formData.slice(
            0,
            request["body"].formData.length - 1,
        );
    };

    const addChecked = (i) => {
        request["body"].fromData[i].checked =
            request["body"].fromData[i].checked;
    };
    const deleteElement = (index) => {
        if (index > -1) {
            request["body"].formData.splice(index, 1); 
        }
        request["body"].fromData = request["body"].formData;
    };
</script>

<h3>Form Fields</h3>
<div class="request-form-data-container">
    {#if request && request["body"] && request["body"].formData}
        {#each request["body"].formData as v, i}
            {#if v.type == "text"}
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
            {/if}
        {/each}
    {/if}
</div>

<button
    on:click|preventDefault={addField}
    class="btn btn-outline-danger"
    title="Add New">Add</button
>
<FormDataFile {request} />

<style>
    .form-data-container {
        margin: 5px;
        display: flex;
        gap: 15px;
    }
</style>
