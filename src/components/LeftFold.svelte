<script>
    import { onMount } from "svelte";
	import Modal from "../Util/Modal.svelte";
	// For Modal
	let showModal = false;
	let files = ['File 1-1', 'File 1-2', 'File 1-3']

	let collections = [{}];
	// Sample JSON data
	// let collectionData = [
	// 	{
	// 		name: 'Collection 1',
	// 		files: ['File 1-1', 'File 1-2', 'File 1-3'],
	// 		isOpen: true // Initialize isOpen
	// 	},
	// 	{
	// 		name: 'Collection 2',
	// 		files: ['File 2-1', 'File 2-2'],
	// 		isOpen: false // Initialize isOpen
	// 	},
	// 	{
	// 		name: 'Collection 3',
	// 		files: ['File 3-1', 'File 3-2', 'File 3-3', 'File 3-4'],
	// 		isOpen: false // Initialize isOpen
	// 	}
	// ];

	// Function to toggle the visibility of files
	// folder.isOpen = true
	function toggleFiles(folder) {
		console.log(folder);
		folder.isOpen = !folder.isOpen;
		collections = [...collections];
	}

  import { invoke } from "@tauri-apps/api/core";
    import Header from "./Header.svelte";

  let response = {};
let name = "";
  async function createCollection() {
    response = await invoke("create_collection", { name });
	collections = [...collections,response]
  }

  

	onMount(async () => {
		collections = await invoke("get_collections");
		// alert(collections);
	});
</script>

<div class="leftFoldContainer">
	<div class="collectionButton">
		<button class="btn btn-outline-danger" on:click={() => (showModal = true)}>New Collection</button>
	</div>
	<div class="collectionList">
		{#each collections as collection}
			<div class="collection">
				<a  on:click={() => toggleFiles(collection)}>
					<i class="folder-icon {collection.isOpen ? 'fas fa-folder-open' : 'fas fa-folder'}"></i>
					{collection.name}
				</a>
				{#if collection.isOpen}
					<div class="files">
						{#each files as file}
							<div class="file">{file}</div>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<Modal bind:showModal>
	<h2 slot="header">
		Collection
	</h2>
	<form on:submit={() => createCollection}>
		<input type="text" placeholder="collection Name" bind:value={name}>
		<button class="btn btn-outline-danger" type="submit" on:click={() => createCollection()}>Create</button>
	</form>
</Modal>

<style>
	.leftFoldContainer{
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
    .collectionButton{
        margin-top: 15px;
        margin-bottom: 15px;
    }
</style>
