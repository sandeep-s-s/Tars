<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import Modal from "../Util/Modal.svelte";
	import { truncateString } from "../Util/util.js";

	import { showToast } from "../storage/toastStore";

	function triggerToast(message) {
		showToast(message);
	}

	let activeRequestUuid = null; // request active class

	export let requestUUid = "";
	// For Modal
	let showCreateCollectionModal = false;

	let showRenameCollectionModal = false;
	let showRenameRequestModal = false;

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
		triggerToast("Collection name modified");
	}

	const onShowPopup = (ev) => {
		showCreateCollectionModal = true;
	};

	const onPopupClose = (data) => {
		showCreateCollectionModal = false;
	};

	function loadRenameRequestModal(request) {
		request = request;
		showRenameRequestModal = true;
		uuid = request.uuid;
		rname = request.name;
	}
	async function renameRequest() {
		response = await invoke("rename_request", { uuid, rname });

		collections = collections.map((collection) => {
			if (collection.id === response.collection_id) {
				const request = collection.requests.find(
					(request) => request.uuid === uuid,
				);
				if (request) {
					request.name = rname;
				}
				return { ...collection };
			}
			return collection;
		});
		showRenameRequestModal = false;
		triggerToast("Request name modified");
	}
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
						<strong>{truncateString(collection.name, 10)} </strong>
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
							<li>
								<a
									class="dropdown-item"
									href="#"
									on:click={() =>
										loadRequestModal(collection.uuid)}
									>New Request</a
								>
							</li>
						</ul>
					</div>
				</div>
				{#if collection.is_open}
					{#each collection.requests as request}
						<div class="list-group-item mx-1">
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
									{activeRequestUuid === request.uuid ? 'text-primary' : ''}"
										>{truncateString(request.name, 7)}</span
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
											<a
												class="dropdown-item"
												href="#"
												on:click={() =>
													loadRenameRequestModal(
														request,
													)}>Rename</a
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

<!-- Create new Request Modal -->

<Modal open={requestFormModel} onClosed={(data) => onPopupClose(data)}>
	<h5 slot="header">New Request</h5>
	<form on:submit={() => createRequest()}>
		<input
			type="text"
			class="form-control"
			placeholder="Request Name"
			bind:value={rname}
		/>
	</form>
	<button
		slot="action"
		class="btn btn-dark btn-sm"
		type="submit"
		on:click={() => createRequest()}>Create</button
	>
</Modal>
<!-- Create Request Modal end here -->

<!-- Rename  Request Modal -->

<Modal open={showRenameRequestModal} onClosed={(data) => onPopupClose(data)}>
	<h5 slot="header">New Request</h5>
	<form on:submit={() => renameRequest()}>
		<input
			type="text"
			class="form-control"
			placeholder="Request Name"
			bind:value={rname}
		/>
	</form>
	<button
		slot="action"
		class="btn btn-dark btn-sm"
		type="submit"
		on:click={() => renameRequest()}>Rename</button
	>
</Modal>

<!-- Rename Request Modal end here -->

<style>
	.list-group-item {
		border: none; /* Remove border */
		padding: 2px;
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
		/* position: fixed; */
		height: 100vh;
		overflow-y: auto;
	}
</style>
