<script>
	export let items = {};
	export let activeTabValue = 0;
	export let request = items["request"];

	const handleClick = (tabValue) => () => {
		activeTabValue = tabValue - 1;
	};
</script>

<div class="request-tab-container">
	<ul  class="request-tabs">
		{#each items["componets"] as item}
			<li class="request-tab-item">
				<a
					class="nav-link {activeTabValue === item.value-1
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
	.request-tabs {
		list-style-type: none;
		margin: 0;
		padding: 0;
		overflow: hidden;
		background-color: #333;
		display: flex;
	}

	.request-tab-item a {
		display: block;
		color: white;
		text-align: center;
		padding: 6px 10px;
		text-decoration: none;
	}


	.request-tab-item a:hover:not(.active) {
		background-color: #111;
	}

	.active {
		background-color:  #C70039 ;
	}
</style>
