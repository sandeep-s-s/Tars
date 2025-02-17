<script>
	import { invoke } from "@tauri-apps/api/core";
	import RequestInput from "./request/RequestInput.svelte";
	import RequestNav from "./request/RequestNav.svelte";

	export let requestUUid = "";
	let request = {};
	export let response = {};

	let request_response = {};

	/**
	 * @type {any[]}
	 */
	export let tabs = [];

	/**
	 * @param {string} uuid
	 */
	async function loadRequest(uuid) {
		request_response = await invoke("get_request", { uuid });
		request = JSON.parse(request_response.request_data);
		const exists = tabs.some(
			(tab) => tab.request_uuid === request_response.uuid,
		);

		tabs = tabs.map((tab) => {
			if (tab.request_uuid === request_response.uuid) {
				return { ...tab, is_active: true };
			}
			return { ...tab, is_active: false };
		});

		if (!exists) {
			let newTab = {
				is_active: true,
				request_name: request.name,
				request_uuid: request_response.uuid,
			};
			tabs = [...tabs, newTab];
		}
	}
	$: loadRequest(requestUUid);
</script>

<div class="mt-3">
	<nav aria-label="breadcrumb">
		<ol class="breadcrumb">
			<li class="breadcrumb-item">
				<strong>{request_response.collection_name}</strong>
			</li>
			<li class="breadcrumb-item active" aria-current="page">
				{request.name}
			</li>
		</ol>
	</nav>
	<RequestInput
		{request}
		{requestUUid}
		bind:method={request.method}
		bind:endpoint={request.endpoint}
		bind:response
	/>
	<RequestNav bind:request />
</div>
