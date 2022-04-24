import { goto } from "$app/navigation";
import { Routes } from "$lib/api";
import { get_store_value } from "svelte/internal";
import { writable, type Writable } from "svelte/store";

export type ClientOptions = {};

export type Description = {
	hint?: string;
	warning?: string;
};

export type Confirm = {
	callback: (identifier: any) => Promise<void>;
	description: Description;
	type_to_confirm?: string;
};

export class Client {
	user: Writable<any>;
	identifier: any;
	token: string;
	authorized: Writable<boolean>;
	confirm: Confirm;

	constructor(_options: ClientOptions) {
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

	async logout() {
		this.user.set(undefined);
		this.authorized.set(false);
		this.token = null;
	}

	async confirm_access(confirm: Confirm) {
		this.confirm = confirm;
		goto("/confirm");
	}
}

export const client = new Client({});
