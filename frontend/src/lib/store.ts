import { writable } from "svelte/store";

export namespace Tokens {
    export const expand = writable({});
    export const tokens = writable({});
}