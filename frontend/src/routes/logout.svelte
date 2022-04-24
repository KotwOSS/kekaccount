<script>
	import { goto } from "$app/navigation";
	import { client } from "$lib/client";

	client.terminate().finally(() => {
		localStorage.removeItem("token");
		goto("/");
	});
</script>

<div class="root fadein" />
