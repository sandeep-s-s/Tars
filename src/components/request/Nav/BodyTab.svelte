<script>
	import XWwwFormUrlencoded from "../Body/XWwwFormUrlencoded.svelte";
	import FormData from "../Body/FormData.svelte";
	import JsonEditor from "../Body/JsonEditor.svelte";
	import TextEditor from "../Body/TextEditor.svelte";
	import XmlEditor from "../Body/XmlEditor.svelte";

	export let request = {};

	let modes = [
		{ id: 1, text: "None", default: true },
		{ id: 2, text: "formData", default: false },
		{ id: 3, text: "x-www-form-urlencoded", default: false },
		{ id: 4, text: "raw", default: false },
	];

	let rawTypes = [
		{ id: 1, text: "Json", default: true },
		{ id: 2, text: "Text", default: false },
		{ id: 3, text: "XML", default: false },
		{ id: 4, text: "HTML", default: false },
	];

	// let request["body"].mode = request["body"].mode; // Default selected mode
	// let request["body"].rawType = ""; // Default selected raw type

	$: if (request["body"].mode === "raw" && !request["body"].rawType) {
		request["body"].rawType = "Json";
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
					bind:group={request["body"].mode}
				/>
				<label for={mode.text}>{mode.text}</label><br />
			</div>
		{/each}
		{#if request["body"].mode === "raw"}
			<div class="raw-type">
				<select
					bind:value={request["body"].rawType}
					class="form-control"
				>
					{#each rawTypes as rawTypeOption}
						<option value={rawTypeOption.text}>
							{rawTypeOption.text}
						</option>
					{/each}
				</select>
			</div>
		{/if}
	</div>

	{#if request["body"].mode === "formData"}
		<div class="formdata-container">
			<FormData {request} />
		</div>
	{/if}
	{#if request["body"].mode === "x-www-form-urlencoded"}
		<div class="formdata-container">
			<XWwwFormUrlencoded {request} />
		</div>
	{/if}
	{#if request["body"].mode === "raw" && request["body"].rawType === "Json"}
		<div class="json-editor">
			<JsonEditor {request} />
		</div>
	{/if}
	{#if request["body"].mode === "raw" && request["body"].rawType === "XML"}
		<div class="json-editor">
			<XmlEditor {request} />
		</div>
	{/if}
	{#if request["body"].mode === "raw" && request["body"].rawType === "Text"}
		<div class="json-editor">
			<TextEditor {request} />
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
	}
</style>
