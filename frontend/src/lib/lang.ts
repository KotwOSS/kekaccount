namespace lang {
	export let language: any;

	export const supported: string[] = ["en_us", "de"];
	export const fallback: string = "en_us";

	export function init() {
		return new Promise((resolve) => {
			if (!language) {
				let preference =
					localStorage.getItem("lang") || navigator.language.replace("-", "_").toLowerCase();
				let fallbacked = supported.includes(preference) ? preference : fallback;

				console.log(fallbacked);

				let xhr = new XMLHttpRequest();
				xhr.responseType = "json";
				xhr.timeout = 2000;

				xhr.onload = () => {
					console.log("Language loaded successfully!");

					language = xhr.response;

					resolve(undefined);
				};

				xhr.open("get", `/lang/${fallbacked}.json`);
				xhr.send();
			}
		});
	}

	export type key =
		| "index.title"
		| "index.description"
		| "nav.title"
		| "nav.dashboard"
		| "nav.login"
		| "nav.register"
		| "nav.home"
		| "nav.title"
		| "nav.title"
		| "nav.title";
}

export default lang;
