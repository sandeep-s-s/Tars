<script>
	export let request = {}
	// let request["body"].raw = ''; // Initial JSON value
	let errorMessage = '';
	let lineCount = 1; // Initialize line count



	function handleInput(event) {
		request["body"].raw = event.target.value;
		errorMessage = ''; // Clear error message on input change
		updateLineCount(); // Update line count on input change
	}

	// Function to update the line count based on the current input
	function updateLineCount() {
		lineCount = request["body"].raw.split('\n').length;
	}
</script>

<div>
	<div class="editor-container">
		<div class="line-numbers">
			{#each Array(lineCount) as _, index}
				{index + 1}
				<br />
			{/each}
		</div>
		<textarea class="editor" bind:value={request["body"].raw} on:input={handleInput} rows="20" />
	</div>
	{#if errorMessage}
		<div class="error">{errorMessage}</div>
	{/if}
</div>

<style>
	.editor-container {
		display: flex;
		border: 1px solid #ccc;
		overflow: hidden;
	}
	.line-numbers {
		background: #f0f0f0;
		padding: 10px;
		text-align: right;
		user-select: none; /* Prevent text selection */
		border-right: 1px solid #ccc;
	}
	.editor {
		flex: 1;
		padding: 10px;
		border: none;
		resize: none;
		font-family: monospace;
		white-space: pre;
		outline: none; /* Remove outline on focus */
	}
	.error {
		color: red;
	}
</style>
