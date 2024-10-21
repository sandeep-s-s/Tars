<script>
    export let authList = [{}];
    export let authType = "";
    console.log("authTuye"+authType)
</script>
<div class="mt-2">
    <label for="authType" class="form-label">Auth Type</label>
    <select id="authType" class="form-control" bind:value={authType}>
        {#each authList as item}
            <option value={item.type}>
                {item.text}
            </option>
        {/each}
    </select>
</div>