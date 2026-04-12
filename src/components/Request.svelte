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
		if (!uuid) return;
		
		const existingData = getByKeyAndUuid(STORAGE_KEY,uuid)
		if(Object.keys(existingData).length === 0){
			try {
				request_response = await invoke("get_request", { uuid });
				request = JSON.parse(request_response.request_data);
				await addOrUpdateByUuid(STORAGE_KEY,request_response)
				window.dispatchEvent(new CustomEvent('tabsRefresh'));
			} catch (error) {
				console.error("Error loading request:", error);
				request_response = {};
				request = {};
			}
		}else{
			request_response = existingData
			if(request_response != null)
				request = JSON.parse(request_response.request_data);
			window.dispatchEvent(new CustomEvent('tabsRefresh'));
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
