<script>
    import { message } from "@tauri-apps/plugin-dialog";
    export let response = {};
    import httpStatusCodes from "../../data/httpStatusCodes.json";

    let lineCount = 0;

    $: lineCount = response.message ? response.message.split("\n").length : 0;
</script>

<div class="container mt-2">
    <div class="editor-container">
        <div class="line-numbers">
            {#each Array(lineCount) as _, i}
                {i + 1}<br />
            {/each}
        </div>
        <div class="text-editor" contenteditable="false">
            {#if response.message}
                {response.message}
            {/if}
        </div>
    </div>
</div>

<style>
    .editor-container {
        display: flex;
        overflow: hidden;
    }
    .line-numbers {
        color: #343434;
        padding: 10px;
        text-align: right;
        user-select: none;
        border-right: 1px solid #ccc;
    }
    .text-editor {
        flex: 1;
        padding: 10px;
        background: #fff;
        overflow: auto;
        white-space: pre;
    }
</style>
