<script lang="ts">
	export let expand: boolean = false;

	function on_click(e) {
		e.preventDefault();
		expand = !expand;
	}
</script>

<div class="category">
	<div class="title">
		<p on:click={on_click}>{expand ? "-" : "+"}</p>
		<slot name="title" />
	</div>
	{#if expand}
		<div class="sub">
			<slot name="sub" />
		</div>
	{/if}
</div>

<style>
	p {
		cursor: pointer;
		user-select: none;
	}

	.category > .title {
		display: flex;
		align-items: center;
		gap: 5px;
		margin-bottom: 5px;
	}

	.category > .sub > :global(div) {
		content: "";
		padding: 5px 0 5px 21px;
		border-left: 1px solid var(--sub-border);
	}

	.category > .sub > :global(div:hover) {
		border-left-color: var(--color);
		border-left-width: 2px;
		padding-left: 20px;
	}

	.category > .sub > :global(div.active) {
		border-left-color: var(--active-color);
		border-left-width: 2px;
		padding-left: 20px;
	}
</style>
