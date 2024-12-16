<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import Modal from "../Util/Modal.svelte";

	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let requestUUid = ""
	// For Modal
	let showModal = false;

	let collections = [{}];
	// Function to toggle the visibility of files
	// folder.isOpen = true
	function toggleCollection(collection) {
		uuid = collection.uuid;
		collection.is_open = !collection.is_open;
		collections = [...collections];
		invoke("toggle_collection",{uuid});
	}

	let response = {};
	let name = "";
	async function createCollection() {
		response = await invoke("create_collection", { name });
		collections = [...collections, response];
	}

	async function getAllCollections(){
		collections = await invoke("get_collections");
	}
	onMount(async () => {
		getAllCollections();
	});

	let requestFormModel = false
	let request_name = ""
	let uuid = ""
	async function createRequest() {
		response = await invoke("create_request",{name,uuid})	
		getAllCollections()
	}

	function loadRequest(requestUuid) {
		requestUUid = requestUuid
		dispatch('requestUuid', requestUUid);
		// console.log(requestUUid)
	}

	function loadRequestModal(collectionUuid){
		requestFormModel = true;
		uuid = collectionUuid
	}
</script>

<div class="leftFoldContainer">
	<div class="collectionButton">
		<button
			class="btn btn-outline-danger"
			on:click={() => (showModal = true)}>New Collection</button
		>
	</div>
	<div class="collection-items">
		{#each collections as collection}
			<div class="collection">
				<div class="collection-logo">
					<i
						class="folder-icon {collection.is_open
							? 'fas fa-folder-open'
							: 'fas fa-folder'}"
					></i>
				</div>
				<div class="collection-name" on:click={() => toggleCollection(collection)}>
					{collection.name} 
				</div>
				<span class="add-request-icon" on:click={() => loadRequestModal(collection.uuid) }><i class="fa fa-plus-square" aria-hidden="true"></i></span>
				{#if collection.is_open}
					<div class="requests">
						{#each collection.requests as request}
							<div class="request">
								<span on:click={() => loadRequest(request.uuid)}>{request.name}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<Modal bind:showModal>
	<h2 slot="header">Collection</h2>
	<form on:submit={() => createCollection()}>
		<input type="text" class="form-control" placeholder="collection Name" bind:value={name} />
		<button
			class="btn btn-outline-danger"
			type="submit"
			>Create</button
		>
	</form>
</Modal>

<Modal bind:showModal={requestFormModel}>
	<h2 slot="header">New Request</h2>
	<form on:submit={() => createRequest}>
		<input type="text"  class="form-control" placeholder="request Name" bind:value={name} />
		<button
			class="btn btn-outline-danger"
			type="submit"
			on:click={() => createRequest()}>Create</button
		>
	</form>
</Modal>


<style>
	.leftFoldContainer {
		margin: 15px;
	}

	.collection {
		display: flex;
		/* justify-content: space-between; */
	}
	.collection {
		cursor: pointer;
		margin: 5px 0;
	}
	.requests {
		padding-left: 20px; 
		padding: .1875rem .5rem;
		margin-top: 1rem;
		margin-left: -150px;
		/* color: rgba(0,0,0,0.65); */
		text-decoration: none;
	}
	.collectionButton {
		margin-top: 15px;
		margin-bottom: 15px;
	}
	.add-request-icon{
		margin-right: 50px;
	}
</style>
