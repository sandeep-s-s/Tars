<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import RequestInput from "./request/RequestInput.svelte";
	import RequestNav from "./request/RequestNav.svelte";

	export let requestUUid = "";
	let request = {};
	export let response = {} ;

	async function loadRequest(uuid) {
		let response = await invoke("get_request", { uuid });
		request = JSON.parse(response);
	}
	$: loadRequest(requestUUid);
</script>

<div class="">
	<RequestInput
		{request} {requestUUid}
		bind:method={request.method}
		bind:endpoint={request.endpoint} bind:response={response}
	/>
	<RequestNav bind:request />
</div>
