<script lang="ts">
	import { afterNavigate } from "$app/navigation";

	import T from "$components/translate.svelte";

	import { LangKey as lk } from "$lib/lang";

	export let detach: boolean = false;

	let path = window.location.pathname;

	afterNavigate(function (navigation) {
		path = navigation.to.pathname;
	});

	let expand_menu: boolean = false;
</script>

{#if detach}
	<div class="spacer" />
{/if}

<div class:detach class="menu" class:expand={expand_menu}>
	<a href="/" class:active={path === "/"}><T k={lk.NAV_HOME} /></a>
	<a href="/dash" class:active={path.startsWith("/dash")}><T k={lk.NAV_DASHBOARD} /></a>
	<a href="/register" class:active={path.startsWith("/register")}><T k={lk.NAV_REGISTER} /></a>
	<a href="/login" class:active={path.startsWith("/login")}><T k={lk.NAV_LOGIN} /></a>
</div>

<nav class:detach>
	<div class="burger" on:click={() => (expand_menu = !expand_menu)}>
		<div />
		<div />
		<div />
	</div>

	<h1><T k={lk.NAV_TITLE} /></h1>
	<div class="links">
		<a href="/" class:active={path === "/"}><T k={lk.NAV_HOME} /></a>
		<a href="/dash" class:active={path.startsWith("/dash")}><T k={lk.NAV_DASHBOARD} /></a>
		<a href="/register" class:active={path.startsWith("/register")}><T k={lk.NAV_REGISTER} /></a>
		<a href="/login" class:active={path.startsWith("/login")}><T k={lk.NAV_LOGIN} /></a>
	</div>
</nav>

<style>
	nav {
		display: flex;
		align-items: center;
		padding: 0 15px;
		height: 50px;
		width: 100vw;
		z-index: 2;
	}

	nav > .links {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		width: calc(100% - 149px);
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

	nav > .burger {
		display: none;
	}

	.menu {
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		transform: translateX(-100%);
		position: absolute;
		top: 50px;
		left: 0;
		width: 100%;
		max-width: 400px;
		height: 100vh;
		background-color: var(--nav-background);
		z-index: 1;
		padding-bottom: 50px;
	}

	.menu.expand.detach {
		position: fixed;
		height: 100vh;
		animation: menudown 0.3s ease forwards;
	}

	.menu.expand {
		transform: translateX(0);
	}

	.spacer {
		height: 50px;
		width: 100%;
	}

	@keyframes menudown {
		0% {
			transform: translateY(-50px);
		}
		100% {
			transform: translateY(0);
		}
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

	@media (max-width: 600px) {
		nav > .burger {
			display: flex;
			flex-direction: column;
			justify-content: space-between;
			width: 30px;
			height: 20px;
			cursor: pointer;
			margin-right: 15px;
		}

		nav > .burger > div {
			width: 100%;
			height: 2px;
			background-color: white;
		}

		nav > .links {
			display: none;
		}
	}

	@media (min-width: 600px) {
		.menu {
			display: none;
		}
	}
</style>
