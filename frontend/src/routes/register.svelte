<script lang="ts">
	import { APIError, hash_password, Routes } from "$lib/api";
	import { regex } from "$lib/checker";
	import { LangKey as lk, language as ln } from "$lib/lang";

	import { goto } from "$app/navigation";

	import { fade } from "svelte/transition";

	const emojis = ["👋😄", "✌️😁", "🤔", "😁👍", "🤔", "😉👍", "😀", "😅", "😀👍", "😁"];
	const emojis_invalid = ["👇😉", "👇😅"];

	let step_index = 0;

	let talking: number = 0;

	let input: any;
	let input_el;

	let error: string | undefined = undefined;

	let text: string;
	let emoji: string;
	$: {
		emoji = error
			? emojis_invalid[Math.floor(Math.random() * emojis_invalid.length)]
			: emojis[step_index];
		text = error ? error : ln["register.keky.step" + step_index];
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

	let email: string;
	let username: string;
	let password: string;

	async function check_email(msg: string) {
		if (!regex.EMAIL.test(msg)) return ln[lk.REGISTER_KEKY_EMAIL_INVALID];
		email = msg;
	}

	async function check_username(msg: string) {
		if (msg.length >= 3 && msg.length <= 32) {
			if (!regex.USERNAME.test(msg)) return ln[lk.REGISTER_KEKY_USERNAME_INVALID];
			try {
				if ((await Routes.Users.SEARCH.send({ name: msg, exact: true })).length !== 0) {
					return ln[lk.REGISTER_KEKY_USERNAME_EXISTS];
				}
			} catch (e) {
				if (e instanceof APIError) {
					return e.get_message();
				} else return ln[lk.ERROR_CONNECTION];
			}
		} else return ln[lk.REGISTER_KEKY_USERNAME_INVALID];
		username = msg;
	}

	async function check_password(msg: string) {
		let weak = regex.WEAK_PASSWORD.test(msg);
		let strong = regex.STRONG_PASSWORD.test(msg);
		let spaces = regex.WHITESPACE.test(msg);

		if (spaces) return ln[lk.REGISTER_KEKY_PASSWORD_SPACE];
		if (msg.length < 8 || (!strong && !weak)) return ln[lk.REGISTER_KEKY_PASSWORD_WEAK];
		password = msg;
	}

	async function check_password_repeat(msg: string) {
		if (msg != password) return ln[lk.REGISTER_KEKY_PASSWORD_MATCH];
	}

	function setup_input() {
		switch (step_index) {
			case 2:
				input = {
					placeholder: ln[lk.REGISTER_KEKY_USERNAME],
					checker: check_username,
					value: username
				};
				break;
			case 4:
				input = {
					placeholder: ln[lk.REGISTER_KEKY_EMAIL],
					checker: check_email,
					value: email
				};
				break;
			case 6:
				input = {
					placeholder: ln[lk.REGISTER_KEKY_PASSWORD],
					checker: check_password,
					type: "password",
					value: password
				};
				break;
			case 7:
				input = {
					placeholder: ln[lk.REGISTER_KEKY_PASSWORD_REPEAT],
					checker: check_password_repeat,
					type: "password"
				};
				break;
			default:
				input = undefined;
		}
	}

	async function next() {
		error = undefined;

		if (input && input.checker) {
			error = await input.checker(input_el.value);
			if (error) return;
			input_el.value = "";
		}

		step_index++;
		setup_input();

		if (step_index === 8) {
			let hashed_password = hash_password(password);
			Routes.Auth.REGISTER.send({
				name: username,
				email,
				password: hashed_password,
				avatar: ""
			})
				.then(() => {
					step_index++;
					setup_input();
				})
				.catch((e) => {
					if (e instanceof APIError) {
						if (e.status === 409) {
							error = ln[lk.REGISTER_KEKY_EMAIL_EXISTS];
							step_index = 4;
							setup_input();
						} else error = e.get_message();
					} else error = ln[lk.ERROR_CONNECTION];
				});
		}
	}

	async function back() {
		error = undefined;

		step_index--;
		setup_input();
	}

	function login() {
		goto("/login");
	}
</script>

<div class="root" in:fade={{ duration: 200 }}>
	<div class="speech">{text.substring(0, text_index)}</div>
	<h1 class="emoji" class:talking>{emoji}</h1>
	{#if input}
		<input
			bind:this={input_el}
			value={input.value || ""}
			type={input.type}
			placeholder={input.placeholder}
		/>
	{/if}
	<div class="controls">
		{#if step_index < 8}
			{#if step_index !== 0}
				<button on:click={back} disabled={talking !== 0} class="back"
					>{ln[lk.REGISTER_KEKY_BACK]}</button
				>
			{/if}
			<button on:click={next} disabled={talking !== 0} class="next"
				>{ln[lk.REGISTER_KEKY_NEXT]}</button
			>
		{/if}
		{#if step_index === 9}
			<button on:click={login}>{ln[lk.NAV_LOGIN]}</button>
		{/if}
	</div>
</div>

<style>
	.controls {
		margin-top: 15px;
		display: flex;
		gap: 10px;
	}

	input {
		margin-top: 10px;
	}

	.root {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
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
