<script>
	import { invoke } from "@tauri-apps/api/core";
	import RequestInput from "./request/RequestInput.svelte";
	import RequestNav from "./request/RequestNav.svelte";
	import { getByKeyAndUuid,addOrUpdateByUuid} from "../storage/storage";
	import {STORAGE_KEY} from "../lib/constants"

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
		const existingData = getByKeyAndUuid(STORAGE_KEY,uuid)
		if(Object.keys(existingData).length === 0){
			request_response = await invoke("get_request", { uuid });
			await updateTabs(request_response)
			await addOrUpdateByUuid(STORAGE_KEY,request_response)
		}else{
			request_response = existingData
			if(request_response != null)
				request = JSON.parse(request_response.request_data);
		}
		updateTabs(request_response)
	}
	$: loadRequest(requestUUid);

	/**
     * @param {{ uuid?: any; id?: any; name?: any; }} request_response
     */
	function updateTabs(request_response){
		const exists = tabs.some(
			(tab) => tab.request_uuid === request_response.uuid,
		);

		tabs = tabs.map((tab) => {
			if (tab.request_id === request_response.id) {
				return { ...tab, is_active: true };
			}
			return { ...tab, is_active: false };
		});

		if (!exists) {
			let newTab = {
				is_active: true,
				request_name: request_response.name,
				request_uuid: request_response.uuid,
				request_id: request_response.id,
			};
			tabs = [...tabs, newTab];
		}
	}


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
