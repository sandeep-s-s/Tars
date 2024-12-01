<script>
    import { open } from "@tauri-apps/plugin-dialog";

    export let request = {};

    let fileGroups = [{ fileName: "", src: "" }];

    function addNewFileGroup() {
        fileGroups = [...fileGroups, { fileName: "", src: "" }];
    }

    function removeFileGroup(index) {
        fileGroups = fileGroups.filter((_, i) => i !== index);
    }

    function handleFileNameChange(event, index) {
        fileGroups[index].fileName = event.target.value;
    }

    if (
        request &&
        request["body"].formData &&
        request["body"].formData.length === 0
    ) {
        request["body"].formData.push({
            key: "",
            value: "",
            checked: false,
            type: "file",
            src: "",
        });
    }
    const addField = () => {
        const formData = request["body"].formData;

        const newObject = {
            key: "",
            value: "",
            type: "file",
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
    async function selectFile(index) {
        const selectedFile = await open({
            multiple: false,
            filters: [{ name: "All Files", extensions: ["*"] }],
        });

        if (selectedFile) {
            request["body"].formData[index].src = selectedFile; // Store the absolute path
        }
    }

    function getFileName(path) {
        return path.split("/").pop();
    }
</script>

<h3>Files</h3>
<div class="request-form-data-container">
    {#if request && request["body"] && request["body"].formData}
        {#each request["body"].formData as v, i}
            {#if v.type == "file"}
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
                        <button on:click={() => selectFile(i)}
                            >Select File</button
                        >
                        {#if v.src}
                            {getFileName(v.src)}
                        {/if}
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

<style>
    .form-data-container {
        margin: 5px;
        display: flex;
        gap: 15px;
    }
</style>
