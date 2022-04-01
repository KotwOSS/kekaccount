<script lang="ts">
	export let error: string = null;
</script>

{#if error}
	<p class="error">{error}</p>
{/if}
