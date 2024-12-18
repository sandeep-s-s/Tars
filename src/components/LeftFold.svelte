<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import Modal from "../Util/Modal.svelte";

	let activeRequestUuid = null; // request active class

	export let requestUUid = "";
	// For Modal
	let showModal = false;

	let collections = [{}];

	function toggleCollection(collection) {
		uuid = collection.uuid;
		collection.is_open = !collection.is_open;
		collections = [...collections];
		invoke("toggle_collection", { uuid });
	}

	export let response = {};
	let name = "";
	async function createCollection() {
		response = await invoke("create_collection", { name });
		collections = [...collections, response];
		showModal = false;
		name = "";
	}

	async function getAllCollections() {
		collections = await invoke("get_collections");
	}
	onMount(async () => {
		getAllCollections();
	});

	let requestFormModel = false;
	let rname = "";
	let uuid = "";
	async function createRequest() {
		response = await invoke("create_request", { rname, uuid });
		getAllCollections();
		rname = "";
		requestFormModel = false;
	}

	function loadRequest(uuid) {
		requestUUid = uuid;
		activeRequestUuid = uuid;
		response = "";
	}

	function loadRequestModal(collectionUuid) {
		requestFormModel = true;
		uuid = collectionUuid;
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
				<div
					class="collection-name"
					on:click={() => toggleCollection(collection)}
				>
					{collection.name}
				</div>
				<span
					class="add-request-icon"
					on:click={() => loadRequestModal(collection.uuid)}
					><i class="fa fa-plus-square" aria-hidden="true"></i></span
				>
				{#if collection.is_open}
					<div class="requests">
						{#each collection.requests as request}
							<div
								class="request {activeRequestUuid ===
								request.uuid
									? 'active'
									: ''} "
							>
								<span on:click={() => loadRequest(request.uuid)}
									>{request.name}</span
								>
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
		<input
			type="text"
			class="form-control"
			placeholder="collection Name"
			bind:value={name}
		/>
		<button class="btn btn-outline-danger" type="submit">Create</button>
	</form>
</Modal>

<Modal bind:showModal={requestFormModel}>
	<h2 slot="header">New Request</h2>
	<form on:submit={() => createRequest}>
		<input
			type="text"
			class="form-control"
			placeholder="request Name"
			bind:value={rname}
		/>
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
	}
	.collection {
		cursor: pointer;
		margin: 5px 0;
	}
	.requests {
		padding-left: 20px;
		padding: 0.1875rem 0.5rem;
		margin-top: 1rem;
		margin-left: -150px;
		/* color: rgba(0,0,0,0.65); */
		text-decoration: none;
	}
	.collectionButton {
		margin-top: 15px;
		margin-bottom: 15px;
	}
	.add-request-icon {
		margin-right: 50px;
	}
	.request.active {
		background-color: #333;
		border-color: #333;
		color: #fff;
	}
</style>
