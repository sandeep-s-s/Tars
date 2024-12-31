<script>
	import { onMount } from "svelte";
	import BasicAuth from "./AuthComponents/BasicAuth.svelte";
	import BearerToken from "./AuthComponents/BearerToken.svelte";

	let selectedOption = "";
	export let request = {};

	const handleOptionChange = (event) => {
		selectedOption = event.target.value;
		updateAuth(); // Update auth data when the option changes
	};

	const updateAuth = () => {
		if (selectedOption === "noauth") request["auth"]["authActive"] = false;
		else {
			request["auth"]["authActive"] = true;
		}
		if (selectedOption === "basic") {
			request["auth"]["authType"] = "basic";
		} else if (selectedOption === "bearer") {
			request["auth"]["authType"] = "bearer";
		}
	};

	onMount(() => {
		selectedOption = request["auth"]["authType"];
		updateAuth(); // Ensure auth data is initialized
	});
</script>

<div class="auth-tab-container">
	<div class="col-4">
		<label for="authType" class="form-label"><strong>Type</strong></label>
		<select
			bind:value={selectedOption}
			on:change={handleOptionChange}
			class="form-control"
		>
			<option value="noauth">No Auth</option>
			<option value="basic">Basic</option>
			<option value="bearer">Bearer</option>
		</select>
	</div>
	<div class="col-8">
		{#if selectedOption === "basic"}
			<BasicAuth bind:auth={request["auth"]} />
		{:else if selectedOption === "bearer"}
			<BearerToken bind:auth={request["auth"]} />
		{/if}
	</div>
</div>

<style>
	.auth-tab-container {
		margin: 5px;
		gap: 15px;
		display: flex;
	}
</style>
