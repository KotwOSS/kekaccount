import { goto } from "$app/navigation";
import { get_login_redirect, Routes } from "$lib/api";
import { get_store_value } from "svelte/internal";
import { writable, type Writable } from "svelte/store";

export type ClientOptions = {};

export class Client {
	user: Writable<any>;
	identifier: any;
	token: string;
	authorized: Writable<boolean>;
	confirm_callback: () => void;

	constructor(options: ClientOptions) {
		this.user = writable(undefined);
		this.authorized = writable(false);
	}

	async login(token: string) {
		this.user.set(await Routes.User.INFO.send({ token }));
		this.authorized.set(true);
		this.token = token;
	}

	async terminate() {
		if (get_store_value(this.authorized)) {
			await Routes.Auth.Token.TERMINATE.send({ token: this.token });
			this.authorized.set(false);
			this.user.set(undefined);
		}
	}

	async confirm_access(confirm_callback: () => void) {
		this.confirm_callback = confirm_callback;
		goto(get_login_redirect());
	}
}

export const client = new Client({});
