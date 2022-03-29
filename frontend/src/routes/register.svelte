<script lang="ts">
	import { Routes } from "$lib/api";

	import { regex } from "$lib/checker";

	import lang from "$lib/lang";

	const emojis = ["👋😄", "✌️😁", "🤔", "🤔", "😀👍"];
	const emojis_invalid = ["👇😉", "👇😅"];

	const steps = 4;
	let step_index = 0;

	let talking: number = 0;

	let input: any;
	let input_value: string;

	let error: string | undefined = undefined;

	let text: string;
	let emoji: string;
	$: {
		emoji = error
			? emojis_invalid[Math.floor(Math.random() * emojis_invalid.length)]
			: emojis[step_index];
		text = error ? error : lang.language["register.keky.step" + step_index];
		talk();
	}

	let text_index = 0;

	function talk() {
		text_index = 0;
		if (talking++ == 0) {
			const intv = setInterval(function () {
				text_index++;
				if (text_index == text.length) {
					clearInterval(intv);
					talking = 0;
				}
			}, 20);
		}
	}

	async function check_email(msg: string) {
		return regex.EMAIL.test(msg) ? undefined : lang.language["register.keky.email.invalid"];
	}

	async function check_username(msg: string) {
		if (msg.length >= 3 && msg.length <= 32) {
			if ((await Routes.Users.SEARCH.send({ name: msg, exact: true })).length != 0)
				return lang.language["register.keky.username.exists"];
		} else return lang.language["register.keky.username.invalid"];
	}

	async function next() {
		error = undefined;

		if (input && input.checker) {
			error = await input.checker(input_value);
			if (error) return;
			input_value = "";
		}

		step_index++;
		switch (step_index) {
			case 2:
				input = {
					placeholder: lang.language["register.keky.email"],
					checker: check_email
				};
				break;
			case 3:
				input = {
					placeholder: lang.language["register.keky.username"],
					checker: check_username
				};
				break;
			default:
				input = undefined;
		}
	}
</script>

<div class="root">
	<div class="speech">{text.substring(0, text_index)}</div>
	<h1 class="emoji" class:talking>{emoji}</h1>
	{#if input}
		<input bind:value={input_value} type="text" placeholder={input.placeholder} />
	{/if}
	{#if step_index != steps}
		<button on:click={next} disabled={talking != 0} class="continue">Continue</button>
	{/if}
</div>

<style>
	.continue {
		margin-top: 15px;
	}

	input {
		margin-top: 10px;
	}

	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		background: linear-gradient(180deg, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 1) 50%), url("/bg.jpg");
		background-size: cover;
	}

	.speech {
		border: 1px solid var(--color);
		padding: 10px;
		max-width: 400px;
		border-radius: 15px;
		margin-bottom: 15px;
	}

	@keyframes talking {
		0% {
			transform: translateY(0);
		}
		10% {
			transform: translateY(-3px);
		}
		20% {
			transform: translateY(3px);
		}
		30% {
			transform: translateY(1px);
		}
		40% {
			transform: translateY(2px);
		}
		50% {
			transform: translateY(-1px);
		}
		60% {
			transform: translateY(-2px);
		}
		70% {
			transform: translateY(-3px);
		}
		80% {
			transform: translateY(0px);
		}
		90% {
			transform: translateY(2px);
		}
		100% {
			transform: translateY(1px);
		}
	}

	@keyframes idle {
		0% {
			transform: rotate(0);
		}
		25% {
			transform: rotate(5deg);
		}
		50% {
			transform: rotate(0);
		}
		75% {
			transform: rotate(-5deg);
		}
		100% {
			transform: rotate(0);
		}
	}

	.emoji {
		animation: 1s linear idle infinite;
	}

	.emoji.talking {
		animation: 1s ease talking infinite;
	}
</style>
