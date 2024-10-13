<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import Modal from "../Util/Modal.svelte";

	// For Modal
	let showModal = false;

	let collections = [{}];
	// Function to toggle the visibility of files
	// folder.isOpen = true
	function toggleFiles(collection) {
		collection.is_open = !collection.is_open;
		collections = [...collections];
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
		console.log(collections)
	});

	let requestFormModel = false
	let request_name = ""
	let uuid = "c8056c96-b204-4114-80c1-567ed9d827f4"
	async function createRequest() {
		response = await invoke("create_request",{name,uuid})	
		getAllCollections()
	}
</script>

<div class="leftFoldContainer">
	<div class="collectionButton">
		<button
			class="btn btn-outline-danger"
			on:click={() => (showModal = true)}>New Collection</button
		>
		<button on:click={() => requestFormModel = true}>Open User Info Modal</button>
	</div>
	<div class="collectionList">
		{#each collections as collection}
			<div class="collection">
				<a on:click={() => toggleFiles(collection)}>
					<i
						class="folder-icon {collection.is_open
							? 'fas fa-folder-open'
							: 'fas fa-folder'}"
					></i>
					{collection.name}
				</a>
				{#if collection.is_open}
					<div class="files">
						{#each collection.requests as request}
							<div class="file">{request.name}</div>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<Modal bind:showModal>
	<h2 slot="header">Collection</h2>
	<form on:submit={() => createCollection}>
		<input type="text" placeholder="collection Name" bind:value={name} />
		<button
			class="btn btn-outline-danger"
			type="submit"
			on:click={() => createCollection()}>Create</button
		>
	</form>
</Modal>

<Modal bind:showModal={requestFormModel}>
	<h2 slot="header">New Request</h2>
	<form on:submit={() => createRequest}>
		<input type="text" placeholder="request Name" bind:value={name} />
		<!-- <input type="text" placeholder="request Name" bind:value={request_name} /> -->
		
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
		cursor: pointer;
		margin: 5px 0;
	}
	.file {
		padding-left: 20px; /* Indent files */
		font-size: medium;
		font-weight: 700;
	}
	.collection a {
		font-size: 18px;
		font-weight: 800;
	}
	.collectionButton {
		margin-top: 15px;
		margin-bottom: 15px;
	}
</style>
