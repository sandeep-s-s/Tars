<script>
	import CodeMirrorEditor from "svelte-codemirror-editor";
	import { showToast } from "../../../storage/toastStore";
	import formatter from "html-prettify";
	export let request = {};

	function triggerToast(message) {
		showToast(message);
	}
</script>

<div class="mt-1">
	<button
		class="mb-2 btn btn-sm btn-dark"
		on:click={() => {
			let data = request["body"].raw;
			try {
				const parsedData = formatter(request["body"].raw);
				if (parsedData != "") {
					request["body"].raw = parsedData;
				} else triggerToast("Invalid Html");
			} catch (error) {
				triggerToast("Invalid Html");
			}
		}}>Format Html</button
	>

	<CodeMirrorEditor
		bind:value={request["body"].raw}
		options={{
			lineNumbers: true,
			mode: "text/html",
			theme: "material",
			autoCloseBrackets: true,
			tabSize: 2,
			indentWithTabs: false,
		}}
	/>
</div>
