<script>
	import CodeMirrorEditor from "svelte-codemirror-editor";
	import { showToast } from "../../../storage/toastStore";
	import xmlFormatter from "xml-formatter";
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
				const parsedData = xmlFormatter(request["body"].raw);
				request["body"].raw = parsedData;
			} catch (error) {
				triggerToast("Invalid XML:");
			}
		}}>Format XML</button
	>

	<CodeMirrorEditor
		bind:value={request["body"].raw}
		options={{
			lineNumbers: true,
			mode: "application/xml",
			theme: "material",
			autoCloseBrackets: true,
			tabSize: 2,
			indentWithTabs: false,
		}}
	/>
</div>
