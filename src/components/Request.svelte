<script>
	import { invoke } from "@tauri-apps/api/core";
	import RequestInput from "./request/RequestInput.svelte";
	import RequestNav from "./request/RequestNav.svelte";

	export let requestUUid = "";
	let request = {};
	export let response = {};

	/**
	 * @type {never[]}
	 */
	export let tabs = [];

	async function loadRequest(uuid) {
		let response = await invoke("get_request", { uuid });
		request = JSON.parse(response.request_data);
		let newTab = {
			is_active: true,
			request_name: request.name,
			request_uuid: response.uuid,
		};
		const exists = tabs.some(
			(tab) => tab.request_uuid === newTab.request_uuid,
		);

		tabs = tabs.map((tab) => {
			if (tab.request_uuid === response.uuid) {
				return { ...tab, is_active: true };
			}
			return { ...tab, is_active: false };
		});

		if (!exists) {
			tabs = [...tabs, newTab];
		}
	}
	$: loadRequest(requestUUid);
</script>

<div class="">
	<RequestInput
		{request}
		{requestUUid}
		bind:method={request.method}
		bind:endpoint={request.endpoint}
		bind:response
	/>
	<RequestNav bind:request />
</div>
