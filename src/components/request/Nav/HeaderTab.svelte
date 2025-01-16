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
		request["headers"] = [...request["headers"], { key: "", name: "",checked:true }];
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
			request["headers"].splice(index, 1); 
		}
		request["headers"] = request["headers"];
	};
</script>

<div class="mt-3">
	{#each request["headers"] as v, i}
		<div class="row mt-1">
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
					class="form-check-input"
				/>
			</div>
			<div class="col-2">
				<button
					on:click={(e) => deleteElement(i)}
					class="btn btn-outline-dark btn-sm"
					title="Delete"><i class="bi bi-x-circle-fill"></i></button
				>
			</div>
		</div>
	{/each}

	<div class="d-flex justify-content-end mt-3">
		<button
			on:click|preventDefault={addField}
			class="btn btn-dark btn-sm"
			title="Add New"><i class="bi bi-plus-square-fill"></i> Add</button
		>
	</div>
</div>
