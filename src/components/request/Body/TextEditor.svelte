<script>
	import CodeMirrorEditor from "svelte-codemirror-editor";
	import { showToast } from "../../../storage/toastStore";
	export let request = {};

	function triggerToast(message) {
		showToast(message);
	}
</script>

<div class="mt-1">
	<CodeMirrorEditor
		bind:value={request["body"].raw}
		options={{
			lineNumbers: true,
			mode: "text/plain",
			theme: "material",
			autoCloseBrackets: true,
			tabSize: 2,
			indentWithTabs: false,
		}}
	/>
</div>
