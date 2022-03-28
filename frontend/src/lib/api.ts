import { goto } from "$app/navigation";
import { api_base } from "$lib/config";
import CryptoJS from "crypto-js";

export type Method = "GET" | "POST" | "PUT" | "DELETE";

export class Permission {
	readonly name: string;
	readonly description: string;
	readonly id: number;
	readonly bitmask: number;

	constructor(name: string, description: string, id: number) {
		this.name = name;
		this.description = description;
		this.id = id;
		this.bitmask = 1 << id;
	}
}

export namespace Permissions {
	export const TOKEN_LIST = new Permission("token_list", "List tokens", 0);

	export const APP_CREATE = new Permission("app_create", "Create apps", 1);

	export const APP_DELETE = new Permission("app_delete", "Delete apps", 2);

	export const APP_UPDATE = new Permission("app_update", "Update apps", 3);

	export const APP_LIST = new Permission("app_list", "List apps", 4);

	export const USER_INFO = new Permission("user_info", "Show private self information", 5);

	export const USER_UPDATE = new Permission("user_update", "Update self user", 6);

	export const APP_TOKEN_CREATE = new Permission("app_token_create", "Create app tokens", 7);

	export const APP_TOKEN_DELETE = new Permission("app_token_delete", "Delete app tokens", 8);

	export const APP_TOKEN_LIST = new Permission("app_token_list", "List app tokens", 9);

	export const TOKEN_TERMINATE = new Permission("token_terminate", "Terminate self token", 10);

	export const PERMISSIONS = [
		TOKEN_LIST,
		APP_CREATE,
		APP_DELETE,
		APP_UPDATE,
		APP_LIST,
		USER_INFO,
		USER_UPDATE,
		APP_TOKEN_CREATE,
		APP_TOKEN_DELETE,
		APP_TOKEN_LIST,
		TOKEN_TERMINATE
	];
}

export class Route<A> {
	readonly method: Method;
	readonly body: (args: A) => any;
	readonly headers: (args: A) => HeadersInit;
	readonly process: (response: Response) => any;
	readonly get_url: (args: A) => string;

	constructor(
		method: Method,
		get_url: (args: A) => string,
		body: (args: A) => any,
		headers: (args: A) => any,
		process: (response: Response) => any
	) {
		this.method = method;
		this.body = body;
		this.headers = headers;
		this.process = process;
		this.get_url = get_url;
	}

	static obody<A>(
		method: Method,
		get_url: (args: A) => string,
		body: (args: A) => any,
		process: (response: Response) => any
	) {
		return new Route(method, get_url, body, () => ({}), process);
	}

	static oheaders<A>(
		method: Method,
		get_url: (args: A) => string,
		headers: (args: A) => any,
		process: (response: Response) => any
	) {
		return new Route(method, get_url, () => ({}), headers, process);
	}

	async send(args: A) {
		const body = this.body(args);
		const headers = this.headers(args);
		const url = this.get_url(args);

		let result = await fetch(`${api_base}${url}`, {
			method: this.method,
			headers: {
				"Content-Type": "application/json",
				...headers
			},
			body: body ? JSON.stringify(body) : undefined
		});

		return await this.process(result);
	}
}

export class APIError extends Error {
	readonly status: number;
	readonly json: any;

	constructor(msg: string, status: number, json: any = undefined) {
		super(msg);

		this.status = status;
		this.json = json;

		Object.setPrototypeOf(this, APIError.prototype);
	}

	get_message(): string {
		return this.json ? this.json.message : this.message;
	}
}

export namespace Routes {
	export async function ok_processor(r: Response) {
		if (r.headers.get("Content-Type") === "application/json") {
			let json = await r.json();
			if (r.status === 200) return json;
			else throw new APIError("", r.status, json);
		} else throw new APIError(await r.text(), r.status);
	}

	export namespace User {
		export const INFO = Route.oheaders(
			"POST",
			() => "/user/info",
			(args: { token: string }) => ({ Authorization: args.token }),
			ok_processor
		);

		export const UPDATE = new Route(
			"POST",
			() => "/user/update",
			(args) => ({ ...args.changes }),
			(args: { token: string; changes: any }) => ({ Authorization: args.token }),
			ok_processor
		);

		export const DELETE = Route.obody(
			"POST",
			() => "/user/delete",
			(args: { identifier: any }) => ({ ...args.identifier }),
			ok_processor
		);
	}

	export namespace Users {
		export const RETRIEVE = Route.obody(
			"GET",
			(args) => `/users/${args.id}`,
			(_args: { id: string }) => undefined,
			ok_processor
		);
	}

	export namespace Auth {
		export const VERIFY = Route.obody(
			"POST",
			() => "/auth/verify",
			(args: { id: string; identifier: any }) => ({ id: args.id, ...args.identifier }),
			ok_processor
		);

		export const REGISTER = Route.obody(
			"POST",
			() => "/auth/register",
			(args: { username: string; email: string; password: string; avatar: string }) => args,
			ok_processor
		);

		export const INFO = Route.obody(
			"POST",
			() => "/auth/info",
			(args: { identifier: any }) => ({ ...args.identifier }),
			ok_processor
		);

		export namespace Token {
			export const CREATE = Route.obody(
				"POST",
				() => "/auth/token/create",
				(args: { name: string; permissions: number; identifier: any }) => ({
					name: args.name,
					permissions: args.permissions,
					...args.identifier
				}),
				ok_processor
			);

			export const DELETE = Route.obody(
				"POST",
				() => "/auth/token/delete",
				(args: { id: string; identifier: any }) => ({ id: args.id, ...args.identifier }),
				ok_processor
			);

			export const INFO = Route.oheaders(
				"POST",
				() => "/auth/token/info",
				(args: { token: string }) => ({ Authorization: args.token }),
				ok_processor
			);

			export const LIST = Route.oheaders(
				"POST",
				() => "/auth/token/list",
				(args: { token: string }) => ({ Authorization: args.token }),
				ok_processor
			);

			export const TERMINATE = Route.oheaders(
				"POST",
				() => "/auth/token/terminate",
				(args: { token: string }) => ({ Authorization: args.token }),
				ok_processor
			);
		}
	}
}

export namespace Regex {
	export const EMAIL = /^\w+[\+\.\w-]*@([\w-]+\.)*\w+[\w-]*\.([a-z]{2,18}|\d+)$/g;
}

export async function authorize(): Promise<any> {
    let token = localStorage.getItem("token");
    if(!token) goto(get_login_redirect())
    else return await Routes.User.INFO.send({ token }).catch(e=>goto(get_login_redirect()));
}

export function get_identifier(uoe: string, password: string): any {
	return Regex.EMAIL.test(uoe) ? { email: uoe, password } : { username: uoe, password };
}

export function get_identifier_ls(): any {
	let uoe = localStorage.getItem("uoe");
	let password = localStorage.getItem("password");
	localStorage.removeItem("uoe");
	localStorage.removeItem("password");
	if (uoe && password) {
		return Regex.EMAIL.test(uoe) ? { email: uoe, password } : { username: uoe, password };
	} else return undefined;
}

export function hash_password(password: string): string {
	return CryptoJS.SHA512(password).toString();
}

export function get_login_redirect(eparams: { k: string; v: string }[] = []): string {
	let params = new URLSearchParams();
	params.set("r", window.location.pathname + window.location.search);
	eparams.forEach((e) => params.set(e.k, e.v));
	return "/login?" + params.toString();
}

export async function is_authorized(): Promise<boolean> {
	let token = localStorage.getItem("token");
	if (token) {
		try {
			let info = await Routes.Auth.Token.INFO.send({ token });
			return info.token.perms === -1;
		} catch (e) {
			return false;
		}
	} else return false;
}
