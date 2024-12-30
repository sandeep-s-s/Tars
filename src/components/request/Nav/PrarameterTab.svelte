<script>
	export let request = {};
	// export let params = request["params"]
	if (request && request["params"] && request["params"].length === 0) {
		request["params"].push({
			key: "",
			value: "",
			checked: false,
		});
	}
	const addField = () => {
		request["params"] = [...request["params"], { key: "", value: "" }];
		//parameterList.update((val) => val = values)
	};

	const removeField = () => {
		request["params"] = request["params"].slice(
			0,
			request["params"].length - 1,
		);
		//parameterList.update((val) => val = values)
	};

	const addChecked = (i) => {
		request["params"][i].checked = request["params"][i].checked;
	};
	const deleteElement = (index) => {
		if (index > -1) {
			// only splice array when item is found
			request["params"].splice(index, 1); // 2nd parameter means remove one item only
		}
		request["params"] = request["params"];
		//parameterList.update((val) => val = values)
	};
</script>

<div class="request-parameter-container">
	{#if request && request["params"]}
		{#each request["params"] as v, i}
			<div class="request-parameter">
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
						bind:value={v.value}
						placeholder="value"
						class="form-control form-control-sm"
					/>
				</div>
				<div class="col-1">
					<input
						type="checkbox"
						bind:checked={v.checked}
						on:change={(e) => addChecked(i)}
						title="Select" class="form-check-input"
					/>
				</div>
				<div class="col-2">
					<button
						on:click={(e) => deleteElement(i)}
						class="btn btn-dark btn-sm"
						title="Delete"><i class="bi bi-trash"></i></button
					>
				</div>
			</div>
		{/each}
	{/if}
</div>
<button
	on:click|preventDefault={addField}
	class="btn btn-outline-danger"
	title="Add New">Add</button
>

<style>
	.request-parameter-container {
		margin: 5px;
		gap: 15px;
	}

	.request-parameter {
		margin: 5px;
		display: flex;
		gap: 15px;
	}
</style>
