<script>
	export let request = {};

	if (request["headers"].length === 0) {
		request["headers"].push({
			key: "",
			value: "",
			checked: false,
		});
	}
	const addField = () => {
		request["headers"] = [...request["headers"], { url: "", name: "" }];
	};

	const removeField = () => {
		request["headers"] = request["headers"].slice(
			0,
			request["headers"].length - 1,
		);
	};

	const addChecked = (i) => {
		// headers[i].checked = !headers[i].checked;
	};
	const deleteElement = (index) => {
		if (index > -1) {
			// only splice array when item is found
			request["headers"].splice(index, 1); // 2nd parameter means remove one item only
		}
		request["headers"] = request["headers"];
	};
</script>

<div class="request-header-container">
	{#each request["headers"] as v, i}
		<div class="request-header">
			<div class="col-4">
				<input
					id={i}
					type="text"
					bind:value={v.key}
					placeholder="key"
					class="form-control form-control-sm"
				/>
			</div>
			<div class="col-4">
				<input
					id={i}
					type="text"
					bind:value={request["headers"][i].value}
					placeholder="value"
					class="form-control form-control-sm"
				/>
			</div>
			<div class="col-1">
				<input
					type="checkbox"
					bind:checked={v.checked}
					on:change={(e) => addChecked(i)}
					title="Select"
				/>
			</div>
			<div class="col-2">
				<button
					on:click={(e) => deleteElement(i)}
					class="btn btn-outline-danger"
					title="Delete">Delete</button
				>
			</div>
		</div>
	{/each}
</div>
<button
	on:click|preventDefault={addField}
	class="btn btn-outline-danger"
	title="Add New">Add</button
>

<style>
	.request-header-container {
		margin: 5px;
		gap: 15px;
	}

	.request-header {
		margin: 5px;
		display: flex;
		gap: 15px;
	}
</style>
