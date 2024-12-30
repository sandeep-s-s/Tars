<script>
	export let open = false;
	export let onClosed;

	const modalClose = (data) => {
		open = false;
		if (onClosed) {
			onClosed(data);
		}
	};
</script>

{#if open}
	<div
		class="modal"
		id="sampleModal"
		tabindex="-1"
		role="dialog"
		aria-labelledby="sampleModalLabel"
		aria-hidden={false}
	>
		<div class="modal-dialog" role="document">
			<div class="modal-content">
				<div class="modal-header">
					<slot name="header" />
				</div>
				<div class="modal-body">
					<slot></slot>
				</div>
				<div class="modal-footer">
					<button
						type="button"
						class="btn btn-light btn-sm"
						data-dismiss="modal"
						on:click={() => modalClose("close")}>Close</button
					>

					<slot name="action" />
					<!-- <button
						type="button"
						class="btn btn-primary btn-sm"
						on:click={() => modalClose("save")}>Save changes</button
					> -->
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal {
		display: block;
	}
	.modal-content {
		border-radius: 0;
	}
</style>
