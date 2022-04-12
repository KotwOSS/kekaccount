<script lang="ts">
	import { afterNavigate } from "$app/navigation";

	import { LangKey as lk, language as ln } from "$lib/lang";
	import { client } from "$lib/client";

	let path = window.location.pathname;

	afterNavigate(function (navigation) {
		path = navigation.to.pathname;
	});

	let expand_menu: boolean = false;

	let authorized = client.authorized;

    let html = document.querySelector("html");

    $: html.setAttribute("style", expand_menu?"overflow-y: hidden":"");
</script>

<div on:click={function() {
    expand_menu = false;
}} class="darken" class:expand={expand_menu}></div>

<div class="spacer"></div>

<div class="menu" class:expand={expand_menu}>
	<a href="/" class:active={path === "/"}>🏠 {ln[lk.NAV_HOME]}</a>
	{#if $authorized}
		<a href="/dash" class:active={path.startsWith("/dash")}>✏️ {ln[lk.NAV_DASHBOARD]}</a>
		<a href="/logout" class:active={path.startsWith("/logout")}>⚔️ {ln[lk.NAV_LOGOUT]}</a>
	{:else}
		<a href="/register" class:active={path.startsWith("/register")}>{ln[lk.NAV_REGISTER]}</a>
		<a href="/login" class:active={path.startsWith("/login")}>{ln[lk.NAV_LOGIN]}</a>
	{/if}
</div>

<nav>
	<div class="burger" class:expand={expand_menu} on:click={function () {
        expand_menu = !expand_menu;
    }}>
		<div />
		<div />
		<div />
	</div>

	<h1>{ln[lk.NAV_TITLE]}</h1>
	<div class="links">
		<a href="/" class:active={path === "/"}>{ln[lk.NAV_HOME]}</a>
		{#if $authorized}
			<a href="/dash" class:active={path.startsWith("/dash")}>{ln[lk.NAV_DASHBOARD]}</a>
			<a href="/logout" class:active={path.startsWith("/logout")}>{ln[lk.NAV_LOGOUT]}</a>
		{:else}
			<a href="/register" class:active={path.startsWith("/register")}>{ln[lk.NAV_REGISTER]}</a>
			<a href="/login" class:active={path.startsWith("/login")}>{ln[lk.NAV_LOGIN]}</a>
		{/if}
	</div>
</nav>

<style>
	nav > h1 {
		font-size: 1.6em;
		letter-spacing: 1px;
	}

	nav {
		display: flex;
		align-items: center;
		padding: 0 15px;
		height: 63px;
		width: 100vw;
		z-index: 2;
        top: 0;
		left: 0;
		position: fixed;
	}

	nav > .links {
		font-size: 18px;
		display: flex;
		justify-content: flex-end;
		align-items: center;
		width: calc(100% - 149px);
		gap: 20px;
	}

	nav > .links > a {
		text-decoration: none;
	}

	nav > .burger {
		display: none;
	}

    .spacer {
        height: 63px;
    }

	.menu {
        align-items: flex-start;
        position: fixed;
		font-size: 20px;
		display: flex;
		flex-direction: column;
        row-gap: 20px;
		transform: translateX(-100%);
        padding-top: 20px;
        padding-left: 20px;
		/* position: absolute; */
		top: 63px;
		left: 0;
		width: 100%;
		max-width: 260px;
		height: calc(100vh - 50px);
		background-color: var(--nav-background);
		z-index: 1;
		padding-bottom: 63px;
        transition: transform 0.3s ease;
	}

    .darken {
        position: fixed;
        z-index: 1;
        /* background-color: rgba(0, 0, 0, 0.7);
        background: rgba(0, 0, 0, 0.7) none repeat scroll 0% 0%; */
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
		nav > .burger {
			display: flex;
			flex-direction: column;
			justify-content: space-between;
			width: 30px;
			height: 20px;
			cursor: pointer;
			margin-right: 25px;
		}

		nav > .burger > div {
			width: 100%;
			height: 1.6px;
            transition: all 0.5s ease;
		}

        nav>.burger.expand>div:nth-child(1) {
            transform: translate(0, 9px) rotate(40deg);
        }

        nav>.burger.expand>div:nth-child(2) {
            opacity: 0;
        }

        nav>.burger.expand>div:nth-child(3) {
            transform: translate(0, -9px) rotate(-40deg);
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
