<script>
    import XWwwFormUrlencoded from "../Body/XWwwFormUrlencoded.svelte";
	import FormDAta from "../Body/FormDAta.svelte";
	import JsonEditor from "../Body/JsonEditor.svelte";
	import TextEditor from "../Body/TextEditor.svelte";
	import XmlEditor from "../Body/XmlEditor.svelte";

	export let request = {};
	

	let modes = [
		{ id: 1, text: "None", default: true },
		{ id: 2, text: "form-data", default: false },
		{ id: 3, text: "x-www-form-urlencoded", default: false },
		{ id: 4, text: "raw", default: false },
	];

	let rawTypes = [
		{ id: 1, text: "Json", default: true },
		{ id: 2, text: "Text", default: false },
		{ id: 3, text: "XML", default: false },
		{ id: 4, text: "HTML", default: false },
	];

	let selectedMode = request["body"].mode; // Default selected mode
	let rawType = ""; // Default selected raw type

	$: if (selectedMode === "raw" && !rawType) {
		rawType = "Json";
	}
</script>

<div class="right-fold-container">
	<div class="body-mode">
		{#each modes as mode}
			<div>
				<input
					type="radio"
					id={mode.text}
					name="request-body-type"
					value={mode.text}
					bind:group={selectedMode}
				/>
				<label for={mode.text}>{mode.text}</label><br />
			</div>
		{/each}
		{#if selectedMode === "raw"}
			<div class="raw-type">
				<select bind:value={rawType} class="form-control">
					{#each rawTypes as rawTypeOption}
						<option value={rawTypeOption.text}>
							{rawTypeOption.text}
						</option>
					{/each}
				</select>
			</div>
		{/if}
	</div>

	{#if selectedMode === "form-data"}
		<div class="formdata-container">
			<FormDAta {request} />
		</div>
	{/if}
	{#if selectedMode === "x-www-form-urlencoded"}
		<div class="formdata-container">
			<XWwwFormUrlencoded {request} />
		</div>
	{/if}
	{#if selectedMode === "raw" && rawType === "Json"}
		<div class="json-editor">
			<JsonEditor />
		</div>
	{/if}
	{#if selectedMode === "raw" && rawType === "XML"}
		<div class="json-editor">
			<XmlEditor />
		</div>
	{/if}
	{#if selectedMode === "raw" && rawType === "Text"}
		<div class="json-editor">
			<TextEditor />
		</div>
	{/if}
</div>

<style>
	
	.right-fold-container {
		margin: 15px;
		display: flex;
		flex-direction: column;
	}
	.body-mode {
		display: flex;
	}
	.json-editor {
		margin: 15px;
		/* display: flex; */
	}
</style>
