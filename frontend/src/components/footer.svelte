<script lang="ts">
    let events: any;
    fetch("https://events.kotw.dev/recent.json")
        .then(r => r.json())
        .then(r => events = r);
</script>

<footer>
	<p class="copyright">(C) 2022 KekOnTheWorld</p>
    {#if events}
        <div class="events">
            {#each events as event}
            <div class="event">
                <a href={event.link} target="_blank">{event.name}</a>
            </div>
            {/each}
        </div>
    {/if}
</footer>

<style>
	footer {
		width: 100%;
		padding: 15px;
		display: flex;
        flex-direction: column;
		gap: 20px;
		justify-content: center;
		align-items: center;
	}

    .events {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        flex-wrap: wrap;
        gap: 15px;
    }
</style>
