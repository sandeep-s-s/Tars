<script>
	import PrarameterTab from "./Nav/PrarameterTab.svelte";
	import BodyTab from "./Nav/BodyTab.svelte";
	import RequestTabs from "./Nav/RequestTabs.svelte";
	import HeaderTab from "./Nav/HeaderTab.svelte";
	import AuthTab from "./Nav/AuthTab.svelte";

	export let request = {}

	let items = {}
	 items["componets"]= [
		{ label: "Parameters", value: 1, component: PrarameterTab },
		{ label: "Body", value: 2, component: BodyTab },
		{ label: "Headers", value: 3, component: HeaderTab},
		{ label: "Auth", value: 4, component: AuthTab},
	];
	items["request"] = request
</script>
<div class="mt-2">
	<RequestTabs {items}  bind:request={request}/>
</div>
