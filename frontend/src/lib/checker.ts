export namespace regex {
	export const STRONG_PASSWORD = /(?=.*?[#?!@$%^&*-])/;
	export const POOR_PASSWORD = /[a-z]/;
	export const WEAK_PASSWORD = /(?=.*?[0-9])/;
	export const WHITESPACE = /^$|\s+/;
	export const EMAIL = /^\w+[\+\.\w-]*@([\w-]+\.)*\w+[\w-]*\.([a-z]{2,18}|\d+)$/g;
}
