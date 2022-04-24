<script lang="ts">
	import { afterNavigate } from "$app/navigation";

	import { LangKey as lk, language as ln } from "$lib/lang";
	import { client } from "$lib/client";
	import Link from "$components/link.svelte";
	import Category from "./category.svelte";

	let path = window.location.pathname;

	afterNavigate(function ({ to }) {
		path = to.pathname;
	});

	let expand_menu: boolean = false;

	let authorized = client.authorized;

	let scroller = document.querySelector("#app > .wrapper");
	$: scroller.setAttribute("style", expand_menu ? "overflow-y: hidden" : "");
</script>

<div
	on:click={function () {
		expand_menu = false;
	}}
	class="darken"
	class:expand={expand_menu}
/>

<div class="menu" class:expand={expand_menu}>
	<div class="links">
		<Link href="/" exact>
			🏠 {ln[lk.NAV_HOME]}
		</Link>

		{#if $authorized}
			<Category expand>
				<Link href="/dash" slot="title">
					✏️ {ln[lk.NAV_DASHBOARD]}
				</Link>
				<svelte:fragment slot="sub">
					<Link href="/dash/account/">Account</Link>
					<Link href="/dash/apps/">Apps</Link>
					<Link href="/dash/tokens/">Tokens</Link>
				</svelte:fragment>
			</Category>

			<Link href="/logout">
				⚔️ {ln[lk.NAV_LOGOUT]}
			</Link>
		{:else}
			<div class="link">
				<a href="/register" class:active={path.startsWith("/register")}>{ln[lk.NAV_REGISTER]}</a>
			</div>
			<div class="link">
				<a href="/login" class:active={path.startsWith("/login")}>{ln[lk.NAV_LOGIN]}</a>
			</div>
		{/if}
	</div>
</div>

<nav>
	<div class="wrapper">
		<div
			class="burger"
			class:expand={expand_menu}
			on:click={function () {
				expand_menu = !expand_menu;
			}}
		>
			<div />
			<div />
			<div />
		</div>

		<h1>{ln[lk.NAV_TITLE]}</h1>
		<div class="links">
			<Link href="/" exact>
				🏠 {ln[lk.NAV_HOME]}
			</Link>
			{#if $authorized}
				<Link href="/dash">
					✏️ {ln[lk.NAV_DASHBOARD]}
				</Link>
				<Link href="/logout">
					⚔️ {ln[lk.NAV_LOGOUT]}
				</Link>
			{:else}
				<Link href="/register">
					⚔️ {ln[lk.NAV_REGISTER]}
				</Link>
				<Link href="/login">
					⚔️ {ln[lk.NAV_LOGIN]}
				</Link>
			{/if}
		</div>
	</div>
</nav>

<style>
	nav > .wrapper > h1 {
		font-size: 1.6em;
	}

	nav {
		display: flex;
		justify-content: center;
		height: var(--nav-height);
		width: 100vw;
		z-index: 2;
		top: 0;
		left: 0;
		position: fixed;
	}

	nav > .wrapper {
		display: flex;
		align-items: center;
		padding: 0 15px;
		height: 100%;
		width: 100%;
		max-width: 1300px;
	}

	nav > .wrapper > .links {
		font-size: 18px;
		display: flex;
		justify-content: flex-end;
		align-items: center;
		width: calc(100% - 149px);
		gap: 20px;
	}

	nav > .wrapper > .burger {
		display: none;
	}

	.menu {
		position: fixed;
		font-size: 20px;
		display: none;
		overflow-y: scroll;
		transform: translateX(-100%);
		top: 63px;
		left: 0;
		width: 100%;
		max-width: 260px;
		height: var(--full-height);
		height: calc(100vh - var(--nav-height));
		background-color: var(--nav-background);
		z-index: 1;
		transition: transform 0.3s ease;
	}

	.menu > .links {
		padding-top: 20px;
		padding-left: 20px;
		padding-bottom: 63px;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		row-gap: 20px;
	}

	.darken {
		position: fixed;
		z-index: 1;
		width: 100vw;
		height: 100vh;
		opacity: 0;
		pointer-events: none;
		transition: opacity 0.3s ease;
	}

	.darken.expand {
		opacity: 1;
		pointer-events: all;
	}

	.menu.expand {
		transform: translateX(0);
	}

	@media (max-width: 600px) {
		nav > .wrapper > .burger {
			display: flex;
			flex-direction: column;
			justify-content: space-between;
			width: 30px;
			height: 20px;
			cursor: pointer;
			margin-right: 25px;
		}

		nav > .wrapper > .burger > div {
			width: 100%;
			height: 1.6px;
			transition: all 0.5s ease;
		}

		nav > .wrapper > .burger.expand > div:nth-child(1) {
			transform: translate(0, 9px) rotate(40deg);
		}

		nav > .wrapper > .burger.expand > div:nth-child(2) {
			opacity: 0;
		}

		nav > .wrapper > .burger.expand > div:nth-child(3) {
			transform: translate(0, -9px) rotate(-40deg);
		}

		nav > .wrapper > .links {
			display: none;
		}

		.menu {
			display: block;
		}
	}
</style>
