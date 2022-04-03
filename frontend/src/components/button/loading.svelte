<script lang="ts">
	import Loader from "$components/loader.svelte";

	export let loading: boolean = false;
    export let disabled: boolean = false;
</script>

<button disabled={disabled}>
	{#if loading}
		<Loader />
	{:else}
		<slot />
	{/if}
</button>

<style>
	button > :global(.loader) {
		width: 18px;
		height: 18px;
	}
</style>
