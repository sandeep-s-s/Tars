<script>
	export let items = {};
	export let activeTabValue = 0;
	export let request = items["request"];

	const handleClick = (tabValue) => () => {
		activeTabValue = tabValue - 1;
	};
</script>

<div class="request-tab-container">
	<ul class="nav nav-tabs" role="tablist">
		{#each items["componets"] as item}
			<li class="nav-item">
				<a
					class="nav-link {activeTabValue === item.value - 1
						? 'active'
						: ''} "
					id="request-parameter"
					data-bs-toggle="tab"
					href="#tab-{item.value}"
					on:click={handleClick(item.value)}>{item.label}</a
				>
			</li>
		{/each}
	</ul>

	<div class="tab-content">
		<svelte:component
			this={items["componets"][activeTabValue].component}
			bind:request
		/>
	</div>
</div>

<style>
	.nav-item > .nav-link {
		border: 0;
		color: #555;
		font-weight: 700;
	}

	.nav-item > .nav-link:hover,
	.nav-item > .nav-link.active {
		background-color: #fff;
		/* border-color: #0a58ca; */
		border-style: solid;
		border-width: 2px 0 0 0;
		border-radius: 0;
	}

	.nav-item > .nav-link.active {
		/* color: #0a58ca; */
	}
</style>
