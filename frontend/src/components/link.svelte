<script lang="ts">
	import { afterNavigate } from "$app/navigation";

	export let href: string;
	export let exact: boolean = false;
	export let border: boolean = false;

	let pathname = window.location.pathname;
	afterNavigate(function ({ to }) {
		pathname = to.pathname;
	});

	$: active = exact ? pathname === href : pathname.startsWith(href);

	$: noborder = !border;
</script>

<div class="link" class:active>
	<a {href} class:active class:noborder><slot /></a>
</div>

<style>
	.noborder {
		border: none;
	}
</style>
