<script lang="ts">
	import { Permissions } from "$lib/api";
	import { language as ln, LangKey as lk } from "$lib/lang";

	export let permissions: number;
	let actives: { [key: number]: boolean } = get_actives(permissions);

	let old_perms = permissions;

	$: {
		if (old_perms !== permissions) actives = get_actives(permissions);
		else permissions = get_permissions(actives);
		old_perms = permissions;
	}

	function get_permissions(a) {
		return Permissions.PERMISSIONS.reduce(function (pre, perm) {
			return a[perm.id] ? pre | perm.bitmask : pre;
		}, 0);
	}

	function get_actives(p) {
		return Permissions.PERMISSIONS.reduce(function (pre, perm) {
			pre[perm.id] = (p & perm.bitmask) !== 0;
			return pre;
		}, {});
	}

	function select_all(e) {
		e.preventDefault();
		permissions = -1;
	}

	function deselect_all(e) {
		e.preventDefault();
		permissions = 0;
	}
</script>

<div class="table card fadein">
	<h1 class="title">{ln[lk.TABLE_PERMISSION_TITLE]}</h1>
	<div class="permissions">
		<div class="wrapper">
			{#each Permissions.PERMISSIONS as perm}
				{@const active = actives[perm.id]}
				<div
					class="permission card"
					class:active
					on:click={function () {
						actives[perm.id] = !actives[perm.id];
					}}
				>
					<div class="wrapper">
						<h3>{perm.description}</h3>
						<p>{perm.name}</p>
					</div>
					<input type="checkbox" bind:checked={actives[perm.id]} />
				</div>
			{/each}
		</div>
	</div>
	<div class="controls">
		<button on:click={deselect_all}>{ln[lk.TABLE_PERMISSION_DESELECT_ALL]}</button>
		<button on:click={select_all}>{ln[lk.TABLE_PERMISSION_SELECT_ALL]}</button>
	</div>
	<input type="number" placeholder={ln[lk.TABLE_PERMISSION_TITLE]} bind:value={permissions} />
</div>

<style>
	.table {
		display: flex;
		flex-direction: column;
		gap: 10px;
		justify-content: center;
		align-items: center;
	}

	.permissions .wrapper {
		height: min-content;
		row-gap: 5px;
		display: flex;
		align-items: center;
		flex-direction: column;
		display: flex;
		flex-direction: column;
	}

	.permissions {
		overflow-y: scroll;
		max-height: 300px;
	}

	.permission {
		display: flex;
		width: 100%;
		padding: 2px 15px;
		border-radius: 10px;
		cursor: pointer;
	}

	.permission .wrapper {
		gap: 0;
		margin-right: auto;
		padding-right: 20px;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
	}
</style>
