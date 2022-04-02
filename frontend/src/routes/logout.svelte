<script>
	import { goto } from "$app/navigation";
	import { client } from "$lib/client";

	client.terminate().finally(() => {
		localStorage.removeItem("token");
		goto("/");
	});
</script>

<div class="root fadein" />

<style>
	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		background: linear-gradient(180deg, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 1) 50%), url("/bg.jpg");
		background-size: cover;
	}
</style>
