<script lang="ts">
	import { goto } from "$app/navigation";

	import { Routes } from "$lib/api";
	import { client } from "$lib/client";
	import { LangKey as lk, language as ln } from "$lib/lang";
	import { Tokens } from "$lib/store";

	let { expand, tokens } = Tokens;

	export let data: any[];

	let data_length = data.length;

	let expand_height: {} = {};
</script>

<div class="table">
	{#each data as token, i}
		{@const expanded = $expand[token.id]}
		{@const atoken = $tokens[token.id]}
		<div
			class="token"
			class:expand={expanded}
			class:first={i === 0}
			class:last={i === data_length - 1}
		>
			<div class="inner">
				<div class="wrapper">
					<p class:hint={token.active}>{token.name}</p>
					<p class="id">{token.id}</p>
				</div>
				<button
					on:click={function () {
						$expand[token.id] = !expanded;
					}}
					class="edit"
				>
					{expanded ? "-" : "+"}
				</button>
			</div>
			<div class="expand">
				<div
					class="wrapper"
					bind:clientHeight={expand_height[token.id]}
					style={expanded ? "margin-top: 0;" : `margin-top: -${expand_height[token.id] || 0}px;`}
				>
					{#if atoken}
						<h1>Actual token</h1>
						<p class="atoken break short">{atoken}</p>
					{/if}

					<h3>Permissions</h3>
					<p>{token.perms}</p>
					<button
						on:click={function () {
							client.confirm_access({
								callback: async function (identifier) {
									await Routes.Auth.Token.DELETE.send({
										id: token.id,
										identifier
									});

									goto("/dash/tokens");
								},
								description: {
									hint: ln[lk.TOKENS_DELETE_CONFIRM]
								}
							});
						}}>{ln[lk.TOKENS_DELETE]}</button
					>
				</div>
			</div>
		</div>
	{/each}
</div>

<style>
	.table {
		border: 1px solid var(--card-border);
		border-radius: 15px;
	}

	.edit {
		border-radius: 0;
		font-size: 20px;
	}

	.token.first .edit {
		border-radius: 0 15px 0 0;
	}

	.token.last .edit {
		border-radius: 0 0 15px 0;
	}

	.token.first.last .edit {
		border-radius: 0 15px 15px 0;
	}

	.token.expand.last .edit {
		border-radius: 0;
	}

	.token.expand.first.last .edit {
		border-radius: 0 15px 0 0;
	}

	.token.expand {
		border-bottom: 1px solid var(--card-border);
	}

	.token .expand > .wrapper {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	@keyframes slow {
		from {
			height: 0;
		}
	}

	.token .expand {
		overflow: hidden;
		box-sizing: initial;
		animation: 0.2s slow step-end forwards;
	}

	.token .expand > .wrapper {
		transition: margin 0.2s ease;
	}

	.inner {
		display: flex;
		border-bottom: 1px solid var(--card-border);
		width: 100%;
	}

	.token.last .inner {
		border-bottom: none;
	}

	.token.expand.last .inner {
		border-bottom: 1px solid var(--card-border);
	}

	.token.last {
		border-bottom: none;
	}

	.wrapper {
		width: 100%;
		padding: 10px 20px;
		display: flex;
		gap: 10px;
	}

	.id {
		margin-left: auto;
	}
</style>
