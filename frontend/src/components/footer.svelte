<script lang="ts">
	import { imprint, privacy } from "$lib/config";
	import { fallback as fallback_lang, LangKey as lk, language as ln, supported as supported_langs } from "$lib/lang";
    import { supported as supported_themes, fallback as fallback_theme } from "$lib/themes";

	let preference_lang =
		localStorage.getItem("lang") || navigator.language.replace("-", "_").toLowerCase();
	let fallbacked_lang = supported_langs[preference_lang] ? preference_lang : fallback_lang;

    let preference_theme = localStorage.getItem("theme");
	let fallbacked_theme = supported_themes[preference_theme] ? preference_theme : fallback_theme;

	let events: any;
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        if (xmlHttp.readyState === 4) events = JSON.parse(xmlHttp.response);
    };
    xmlHttp.open("GET", "https://events.kotw.dev/recent.json", true);
    xmlHttp.send(null);
</script>

<footer>
	<div class="wrapper">
		<div class="big">
			<p class="emoji">👋</p>
			<p class="short">{@html ln[lk.FOOTER_NOTICE]}</p>
		</div>
		<div class="categories">
			<div class="legal category">
				<h4>⚖️ {ln[lk.FOOTER_LEGAL]}</h4>
				{#if privacy}<a href={privacy}>🔒 {ln[lk.FOOTER_PRIVACY]}</a>{/if}
				{#if imprint}<a href={imprint}>📖 {ln[lk.FOOTER_IMPRINT]}</a>{/if}
			</div>

			<div class="language category">
				<h4>🗣️ {ln[lk.FOOTER_LANGUAGE]}</h4>
				{#each Object.entries(supported_langs) as lang}
					<!-- svelte-ignore a11y-invalid-attribute -->
					<a
						class:active={fallbacked_lang === lang[0]}
						on:click={function (e) {
							e.preventDefault();
							if (fallbacked_lang === lang[0]) return;

							localStorage.setItem("lang", lang[0]);
							window.location = window.location; // Causes the page to reload
						}}
						href="#">{lang[1]}</a
					>
				{/each}
			</div>

            <div class="theme category">
				<h4>🌈 {ln[lk.FOOTER_THEME]}</h4>
				{#each Object.entries(supported_themes) as theme}
					<!-- svelte-ignore a11y-invalid-attribute -->
					<a
						class:active={fallbacked_theme === theme[0]}
						on:click={function (e) {
							e.preventDefault();
							if (fallbacked_theme === theme[0]) return;

							localStorage.setItem("theme", theme[0]);
							window.location = window.location; // Causes the page to reload
						}}
						href="#">{ln[theme[1]]}</a
					>
				{/each}
			</div>

			{#if events}
				<div class="events category">
					<h4>🏅 {ln[lk.FOOTER_EVENTS]}</h4>
					{#each events as event}
						<div class="event">
							<a href={event.link} target="_blank">{event.name}</a>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
	<p class="copyright">(C) 2022 KekOnTheWorld</p>
</footer>

<style>
	.big > .emoji {
		font-size: 70px;
		margin-bottom: 5px;
	}

	.big {
		padding: 30px;
		margin-bottom: 10px;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	footer {
		padding: 10px 0;
	}

	.wrapper {
		width: 100%;
		padding: 15px;
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 20px;
		justify-content: space-around;
		margin-bottom: 30px;
	}

	.wrapper > .categories {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 20px;
		column-gap: 100px;
		padding: 0 15px;
		margin-top: 20px;
	}

	.category * {
		text-align: left;
	}

	.category {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}
</style>
