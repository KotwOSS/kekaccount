import { LangKey as lk } from "$lib/lang";

export type Theme = {
    name: lk,
    emoji: string
};

export const supported: { [key: string]: Theme } = {
	dark: {
        name: lk.THEME_DARK,
        emoji: "⚫"
    },
	light: {
        name: lk.THEME_LIGHT,
        emoji: "⚪"
    }
};

export const fallback: string = "dark";