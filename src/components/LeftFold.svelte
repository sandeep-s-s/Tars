<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import Modal from "../Util/Modal.svelte";

	let activeRequestUuid = null; // request active class

	export let requestUUid = "";
	// For Modal
	let showCreateCollectionModal = false;

	let showRenameCollectionModal = false;

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
		showCreateCollectionModal = false;
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

	function renameCollectionModal(collection) {
		showRenameCollectionModal = true;
		uuid = collection.uuid;
		name = collection.name;
	}

	async function renameCollection() {
		response = await invoke("rename_collection", { uuid, name });

		collections = collections.map((item) => {
			if (item.uuid === uuid) {
				return { ...item, name: name }; 
			}
			return item; 
		});
		showRenameCollectionModal = false;
		name = "";
	}
</script>

<div class="leftFoldContainer">
	<div class="collectionButton">
		<button
			class="btn btn-outline-danger"
			on:click={() => (showCreateCollectionModal = true)}
			>New Collection</button
		>
	</div>
	<div class="collection-items">
		{#each collections as collection}
			<div class="collection">
				<div class="collection-icon">
					<i
						class="folder-icon {collection.is_open
							? 'fas fa-folder-open'
							: 'fas fa-folder'}"
					></i>
				</div>
				<div class="collection-name-container">
					<div
						class="collection-name"
						on:click={() => toggleCollection(collection)}
					>
						{collection.name}
					</div>
					<div class="collection-settings-icon">
						<div class="dropdown" style="float:left;">
							<i class="fa fa-plus-square" aria-hidden="true"></i>
							<div class="dropdown-content" style="left:0;">
								<a
									href="#"
									on:click={() =>
										loadRequestModal(collection.uuid)}
									>New Request</a
								>
								<a
									href="#"
									on:click={() =>
										renameCollectionModal(collection)}
									>Rename</a
								>
							</div>
						</div>
					</div>
				</div>
				{#if collection.is_open}
					<div class="request-name-container">
						{#each collection.requests as request}
							<div
								class="request {activeRequestUuid ===
								request.uuid
									? 'active'
									: ''} "
							>
								<div on:click={() => loadRequest(request.uuid)}
									>{request.name}</div
								>
								<!-- <div class="collection-settings-icon">
									<div class="dropdown" style="float:left;">
										<i class="fa fa-plus-square" aria-hidden="true"></i>
										<div class="dropdown-content" style="left:0;">
											<a
												href="#"
												on:click={() =>
													loadRequestModal(collection.uuid)}
												>New Request</a
											>
											<a
												href="#"
												on:click={() =>
													renameCollectionModal(collection)}
												>Rename</a
											>
										</div>
									</div>
								</div> -->
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<!--  Create Collection Modal -->
<Modal bind:showModal={showCreateCollectionModal}>
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
<!-- Create Collection Modal end here -->

<!--  Modify Collection Modal -->
<Modal bind:showModal={showRenameCollectionModal}>
	<h2 slot="header">Collection</h2>
	<form on:submit={() => renameCollection()}>
		<input
			type="text"
			class="form-control"
			placeholder="collection Name"
			bind:value={name}
		/>
		<button class="btn btn-outline-danger" type="submit">Rename</button>
	</form>
</Modal>
<!-- Modify Collection Modal end here -->

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
		cursor: pointer;
		margin: 5px 0;
	}
	.collection-icon {
		flex: 1;
	}
	.collection-name-container {
		flex: 10;
		display: flex;
	}
	.collection-name {
		flex: 9;
	}

	.collection-settings-icon {
		margin-right: 50px;
	}

	/* .requests {
		padding-left: 20px;
		padding: 0.1875rem 0.5rem;
		margin-top: 1rem;
	
		text-decoration: none;
	} */
	.collectionButton {
		margin-top: 15px;
		margin-bottom: 15px;
	}
	.request.active {
		background-color: #333;
		border-color: #333;
		color: #fff;
	}
	.request-name-container {
		/* display: flex; */
		/* flex-direction: row; */
	}
</style>
