<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import RequestInput from "./request/RequestInput.svelte";
	import RequestNav from "./request/RequestNav.svelte";

	export let requestUUid = "";
	let request = {};


	async function loadRequest(uuid) {
		let response = await invoke("get_request", { uuid });
		request = JSON.parse(response);
	}
	$: loadRequest(requestUUid);
</script>

<div class="col">
	<!-- <pre>{JSON.stringify(request, null, 1)}</pre> -->
	<RequestInput
		{request} {requestUUid}
		bind:method={request.method}
		bind:endpoint={request.endpoint}
	/>
	<RequestNav bind:request />
</div>
