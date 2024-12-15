<script>
	import Body from "./Body.svelte";
	import Headers from "./Headers.svelte";
	import ResponseTabs from "./ResponseTabs.svelte";

	export let response = {};

	let items = {};
	items["componets"] = [
		{ label: "Response", value: 1, component: Body },
		{ label: "Headers", value: 2, component: Headers },
	];
	items["response"] = response;
</script>

<div class="response-tab-container">
	<ResponseTabs {items} bind:response />
</div>

<style>
	.response-tab-container {
		margin-top: 20px;
	}
</style>
