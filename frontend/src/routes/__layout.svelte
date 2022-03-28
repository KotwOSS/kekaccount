<script lang="ts">
	import Loader from "$components/loader.svelte";
	import Navbar from "$components/navbar.svelte";
	import Footer from "$components/footer.svelte";
	import lang from "$lib/lang";

	let loading: boolean = true;

	async function main() {
		await lang.init();
		loading = false;
	}

	main();

	function onscroll(e) {
		let scroll = e.target.scrollTop;
		detach_navbar = scroll >= 50;
	}

	let detach_navbar = false;
</script>

<div id="app" class="fadein" on:scroll={onscroll}>
	{#if loading}
		<Loader />
	{:else}
		<Navbar detach={detach_navbar} />
		<slot />
		<Footer />
	{/if}
</div>

<style>
	#app > :global(.loader) {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
	}

	#app {
		height: 100vh;
		overflow-y: scroll;
	}
</style>
