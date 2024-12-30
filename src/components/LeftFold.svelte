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
		// showPopup = false;
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

	const onShowPopup = (ev) => {
		showCreateCollectionModal = true;
	};

	const onPopupClose = (data) => {
		showCreateCollectionModal = false;
	};
</script>

<div class="sidebar" style="border-right: 1px solid #ccc;">
	<div class="collection"></div>
	<div class="mt-2">
		<h5>Collections</h5>
	</div>
	<hr />
	<div class="collection">
		<!-- <ul class="nav d-flex justify-content-end"> -->
		<ul class="nav">
			<li class="nav-item">
				<button class="btn btn-dark btn-sm" on:click={onShowPopup}>
					<i class="bi bi-plus-square"></i> New Collection
				</button>
			</li>
		</ul>
	</div>
	<div class="list-group mt-2">
		<!-- {#each data.slice(0, 4) as folder, index} -->
		{#each collections as collection, index}
			<!-- Limit to 4 folders -->
			<div class="list-group-item">
				<div class="d-flex justify-content-between align-items-center">
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<!-- svelte-ignore a11y-no-static-element-interactions -->
					<!-- svelte-ignore a11y-missing-attribute -->
					<a class="" on:click={() => toggleCollection(collection)}>
						<i
							class="folder-icon {collection.is_open
								? 'bi bi-folder2-open'
								: 'bi bi-folder-fill'}"
						>
						</i>
						<strong>{collection.name} </strong>
					</a>
					<div class="dropdown">
						<button
							type="button"
							class="btn"
							data-bs-toggle="dropdown"
						>
							<i class="bi bi-three-dots-vertical"></i>
						</button>
						<ul class="dropdown-menu">
							<li>
								<a
									class="dropdown-item"
									href="#"
									on:click={() =>
										renameCollectionModal(collection)}
									>Rename Collection</a
								>
							</li>
						</ul>
					</div>
				</div>
				{#if collection.is_open}
					{#each collection.requests as request}
						<div class="list-group-item">
							<div
								class="d-flex justify-content-between align-items-center active"
							>
								<!-- svelte-ignore a11y-click-events-have-key-events -->
								<!-- svelte-ignore a11y-no-static-element-interactions -->
								<!-- svelte-ignore a11y-missing-attribute -->
								<a
									class=""
									on:click={() => loadRequest(request.uuid)}
								>
									<i class="bi bi-file-earmark-text-fill"></i>
									<span
										class="
									{activeRequestUuid === request.uuid ? 'text-primary' : ''}">{request.name}</span
									>
								</a>
								<div class="dropdown">
									<button
										type="button"
										class="btn"
										data-bs-toggle="dropdown"
									>
										<i class="bi bi-three-dots-vertical"
										></i>
									</button>
									<ul class="dropdown-menu">
										<li>
											<a class="dropdown-item" href="#"
												>Link 1</a
											>
										</li>
									</ul>
								</div>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		{/each}
	</div>
</div>

<!-- <div class="leftFoldContainer">
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
								<div on:click={() => loadRequest(request.uuid)}>
									{request.name}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div> -->

<!-- Create Collection Modal -->
<Modal open={showCreateCollectionModal} onClosed={(data) => onPopupClose(data)}>
	<h5 slot="header">New Collection</h5>
	<form on:submit={() => createCollection()}>
		<input
			type="text"
			class="form-control"
			placeholder="collection Name"
			bind:value={name}
		/>
	</form>
	<button
		slot="action"
		class="btn btn-dark btn-sm"
		type="submit"
		on:click={() => createCollection()}>Create</button
	>
</Modal>
<!-- Create Collection Modal ends here -->

<!-- Rename Collection Modal -->
<Modal open={showRenameCollectionModal} onClosed={(data) => onPopupClose(data)}>
	<h5 slot="header">Rename Collection</h5>
	<form on:submit={() => renameCollection()}>
		<input
			type="text"
			class="form-control"
			placeholder="collection Name"
			bind:value={name}
		/>
	</form>
	<button
		slot="action"
		class="btn btn-dark btn-sm"
		type="submit"
		on:click={() => renameCollection()}>Rename</button
	>
</Modal>

<!-- Rename Collection Modal ends here -->
<style>
	.list-group-item {
		border: none; /* Remove border */
	}
	a,
	a:hover,
	a:focus,
	a:active {
		text-decoration: none;
		color: inherit;
		cursor: pointer;
	}

	.sidebar {
		height: 100%;
		background-color: #f8f9fa;
		padding-top: 20px;
		position: fixed;
	}
	.active {
		/* background-color: #777; */
	}
</style>
