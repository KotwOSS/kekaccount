<script lang="ts">
	import T from "$components/translate.svelte";

	export let detach: boolean = false;

	let path = window.location.pathname;
</script>

{#if detach}
	<div class="spacer" />
{/if}
<nav class:detach>
	<h1><T k="nav.title" /></h1>
	<div class="links">
		<a href="/" class:active={path === "/"}><T k="nav.home" /></a>
		<a href="/dash"><T k="nav.dashboard" /></a>
		<a href="/register"><T k="nav.register" /></a>
		<a href="/login"><T k="nav.login" /></a>
	</div>
</nav>

<style>
	nav {
		display: flex;
		align-items: center;
		padding: 0 15px;
		height: 50px;
		width: 100%;
		z-index: 1;
		transition: all 0.5s ease;
	}

	nav > .links {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		width: 100%;
		gap: 15px;
	}

	nav > .links > a {
		text-decoration: none;
	}

	nav.detach {
		top: 0;
		left: 0;
		position: fixed;
		animation: navdown 0.3s ease forwards;
	}

	.spacer {
		height: 50px;
		width: 100%;
	}

	@keyframes navdown {
		0% {
			opacity: 0;
			transform: translateY(-100%);
		}
		100% {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
