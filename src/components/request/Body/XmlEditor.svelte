<script>
	export let request = {}
	// let xmlInput = '<root><key>value</key></root>'; // Initial XML value
	let errorMessage = '';
	let lineCount = 1; // Initialize line count

	function formatXml() {
		const parser = new DOMParser();
		const xmlDoc = parser.parseFromString(request["body"].raw, 'text/xml');
		
		// Check for parsing errors
		const parseError = xmlDoc.getElementsByTagName('parsererror');
		if (parseError.length > 0) {
			errorMessage = 'Invalid XML: ' + parseError[0].textContent;
			return; // Exit the function if there's an error
		}

		// If no errors, format the XML
		const serializer = new XMLSerializer();
		const formattedXml = serializer.serializeToString(xmlDoc);
		request["body"].raw = formattedXml.replace(/></g, '>\n<'); // Add new lines for better readability
		errorMessage = ''; // Clear any previous error messages
		updateLineCount(); // Update line count after formatting
	}

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
	<button on:click={formatXml} class="btn btn-sm btn-dark my-2">Format XML</button>
	<div class="editor-container">
		<div class="line-numbers">
			{#each Array(lineCount) as _, index}
				{index + 1}
				<br />
			{/each}
		</div>
		<textarea class="editor form-control" bind:value={request["body"].raw} on:input={handleInput} rows="20" />
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
		margin-top: 10px; /* Add some space above the error message */
	}
</style>
