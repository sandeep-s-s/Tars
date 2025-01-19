<script>
	import CodeMirrorEditor from "svelte-codemirror-editor";
	import { showToast } from "../../../storage/toastStore";
	export let request = {};

	function triggerToast(message) {
		showToast(message);
	}
</script>

<div class="mt-1">
	<button
		class="mb-2 btn btn-sm btn-dark"
		on:click={() => {
			try {
				const parsedData = JSON.parse(request["body"].raw);
				request["body"].raw = JSON.stringify(parsedData, null, 2);
			} catch (error) {
				triggerToast("Invalid JSON:");
			}
		}}>Format JSON</button
	>

	<CodeMirrorEditor
		bind:value={request["body"].raw}
		options={{
			lineNumbers: true,
			mode: "application/json",
			theme: "material",
			autoCloseBrackets: true,
			tabSize: 2,
			indentWithTabs: false,
		}}
	/>
</div>
