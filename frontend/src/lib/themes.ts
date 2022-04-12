import { LangKey as lk } from "$lib/lang";

export const supported: { [key: string]: lk } = {
	dark: lk.THEME_DARK,
	light: lk.THEME_LIGHT,
};
export const fallback: string = "dark";