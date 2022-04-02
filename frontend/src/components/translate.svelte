<script lang="ts">
	import { LangKey, language } from "$lib/lang";

	export let k: LangKey;
</script>

{@html language[k]}
