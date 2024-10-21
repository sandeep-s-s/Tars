<script>
	
	export let request = {}
	// export let request["headers"] = request["headers"]

	
	if (request["headers"].length === 0){
		request["headers"].push({
			key: '',
			value: '',
			checked: false
		});
	}
	const addField = () => {
		request["headers"] = [...request["headers"], { url: '', name: '' }];
	};

	const removeField = () => {
		request["headers"] = request["headers"].slice(0, request["headers"].length - 1);
	};

	const addChecked = (i) => {
		// headers[i].checked = !headers[i].checked;
	};
	const deleteElement = (index) => {
		if (index > -1) { // only splice array when item is found
			request["headers"].splice(index, 1); // 2nd parameter means remove one item only
		}
		request["headers"] = request["headers"]
	};
</script>

{#each request["headers"] as v, i}
	<div class="mt-3 row">
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
			<input type="checkbox"  bind:checked={v.checked} on:change={(e) => addChecked(i)} title="Select"/>
		</div>
		<div class="col-2">
			<button on:click={(e) => deleteElement(i)} class="btn btn-sm btn-danger float-left" title="Delete"><i class="bi bi-trash3"></i></button>
		</div>
	</div>
{/each}

<div class="tab-pane fade show active" id="tab-parameter">
	<div class="container mt-5 d-grid gap-2 d-md-flex justify-content-md-end">
		<button on:click|preventDefault={addField} class="btn btn-sm btn-primary" title="Add New"><i class="bi bi-plus-square"></i></button>
	</div>
	<div id="textbox-container"></div>
</div>
