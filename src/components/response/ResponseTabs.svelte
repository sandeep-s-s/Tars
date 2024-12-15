<script>
    export let items = {};
    export let activeTabValue = 0;
    export let response = items["response"];

    const handleClick = (tabValue) => () => {
        activeTabValue = tabValue - 1;
    };
</script>

<div class="response-tab-container">
    <ul class="response-tabs">
        {#each items["componets"] as item}
            <li class="response-tab-item">
                <a
                    class="nav-link {activeTabValue === item.value - 1
                        ? 'active'
                        : ''} "
                    id="response-parameter"
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
            bind:response
        />
    </div>
</div>

<style>
    .response-tabs {
        list-style-type: none;
        margin: 0;
        padding: 0;
        overflow: hidden;
        background-color: #333;
        display: flex;
    }

    .response-tab-item a {
        display: block;
        color: white;
        text-align: center;
        padding: 6px 10px;
        text-decoration: none;
    }

    .response-tab-item a:hover:not(.active) {
        background-color: #111;
    }

    .active {
        background-color: #04aa6d;
    }
</style>
