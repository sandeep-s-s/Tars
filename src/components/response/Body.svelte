<script>
    import CodeMirrorEditor from "svelte-codemirror-editor";
    export let response = {};
    import httpStatusCodes from "../../data/httpStatusCodes.json";
    import { readonly } from "svelte/store";

    $: jsonData = response.message;
</script>

<div class="container mt-2">
    {#if jsonData}
        
    <p><strong>Status : {response["status_code"]} {httpStatusCodes[response["status_code"]]}</strong></p>
    <CodeMirrorEditor
        bind:value={jsonData}
        options={{
            lineNumbers: true,
            mode: "application/json", // Set mode to JSON
            theme: "material", // Use the theme you imported
            autoCloseBrackets: true, // Enable auto-closing brackets
            tabSize: 2, // Set tab size for indentation
            indentWithTabs: false, // Use spaces instead of tabs
        }}
        readonly
    />
    {/if}
</div>
