<script lang="ts">
    import { imprint, privacy } from "$lib/config";
    import T from "$components/translate.svelte";
    import { LangKey as lk } from "$lib/lang";

	let events: any;
	fetch("https://events.kotw.dev/recent.json")
		.then((r) => r.json())
		.then((r) => (events = r));
</script>

<footer>
    <div class="wrapper">
        <div class="big">
            <p class="emoji">👋</p>
            <p class="short">This awesome opensource account system was made by KekOnTheWorld. Check it out on <a href="https://github.com/KotwOSS/kekaccount" target="_blank">Github</a>!</p>
        </div>
        <div class="categories">
            <div class="legal category">
                <h4>Legal</h4>
                {#if privacy}<a href={privacy}><T k={lk.FOOTER_PRIVACY} /></a>{/if}
                {#if imprint}<a href={imprint}><T k={lk.FOOTER_IMPRINT} /></a>{/if}
            </div>
            
            {#if events}
            <div class="events category">
                <h4>Events</h4>
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
    .big>.emoji {
        font-size: 70px;
        margin-bottom: 5px;
    }

    .big {
        padding: 30px;
        margin-bottom: 20px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        /* border: 1px solid var(--card-border); */
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
		/* align-items: center; */
	}

    .wrapper>.categories {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 20px;
        column-gap: 100px;
        align-items: center;
        padding: 0 15px;
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
