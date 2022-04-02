<script lang="ts">
	import { imprint, privacy } from "$lib/config";
	import T from "$components/translate.svelte";
	import { fallback, LangKey as lk, supported } from "$lib/lang";
    import { goto } from "$app/navigation";

    let preference =
        localStorage.getItem("lang") || navigator.language.replace("-", "_").toLowerCase();
    let fallbacked = supported[preference] ? preference : fallback;

	let events: any;
	fetch("https://events.kotw.dev/recent.json")
		.then((r) => r.json())
		.then((r) => (events = r));
</script>

<footer>
	<div class="wrapper">
		<div class="big">
			<p class="emoji">👋</p>
			<p class="short"><T k={lk.FOOTER_NOTICE} /></p>
		</div>
		<div class="categories">
			<div class="legal category">
				<h4>⚖️ <T k={lk.FOOTER_LEGAL} /></h4>
				{#if privacy}<a href={privacy}>🔒 <T k={lk.FOOTER_PRIVACY} /></a>{/if}
				{#if imprint}<a href={imprint}>📖 <T k={lk.FOOTER_IMPRINT} /></a>{/if}
			</div>

            <div class="language category">
                <h4>🗣️ <T k={lk.FOOTER_LANGUAGE} /></h4>
                {#each Object.entries(supported) as lang}
                    <!-- svelte-ignore a11y-invalid-attribute -->
                    <a class:active={fallbacked === lang[0]} on:click={function(e) {
                        e.preventDefault();
                        if(fallbacked === lang[0]) return;

                        localStorage.setItem("lang", lang[0]);
                        window.location = window.location; // Causes the page to reload 
                    }} href="#">{lang[1]}</a>
                {/each}
            </div>

			{#if events}
				<div class="events category">
					<h4>🏅 <T k={lk.FOOTER_EVENTS} /></h4>
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
