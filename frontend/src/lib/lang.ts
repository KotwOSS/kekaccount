export let language: any;
export let initialized: number = 0;

export const supported: string[] = ["en_us", "de"];
export const fallback: string = "en_us";

export function init() {
	return new Promise((resolve) => {
		if (initialized++ === 0) {
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

export enum LangKey {
	// INDEX
	INDEX_TITLE = "index.title",
	INDEX_DESCRIPTION = "index.description",
	// NAV
	NAV_TITLE = "nav.title",
	NAV_DASHBOARD = "nav.dashboard",
	NAV_LOGIN = "nav.login",
	NAV_REGISTER = "nav.register",
	NAV_HOME = "nav.home",
	// REGISTER
	REGISTER_KEKY_NEXT = "register.keky.next",
	REGISTER_KEKY_BACK = "register.keky.back",
	REGISTER_KEKY_STEP0 = "register.keky.step0",
	REGISTER_KEKY_STEP1 = "register.keky.step1",
	REGISTER_KEKY_STEP2 = "register.keky.step2",
	REGISTER_KEKY_STEP3 = "register.keky.step3",
	REGISTER_KEKY_STEP4 = "register.keky.step4",
	REGISTER_KEKY_STEP5 = "register.keky.step5",
	REGISTER_KEKY_STEP6 = "register.keky.step6",
	REGISTER_KEKY_STEP7 = "register.keky.step7",
	REGISTER_KEKY_STEP8 = "register.keky.step8",
	REGISTER_KEKY_EMAIL = "register.keky.email",
	REGISTER_KEKY_EMAIL_EXISTS = "register.keky.email.exists",
	REGISTER_KEKY_EMAIL_INVALID = "register.keky.email.invalid",
	REGISTER_KEKY_USERNAME = "register.keky.username",
	REGISTER_KEKY_USERNAME_INVALID = "register.keky.username.invalid",
	REGISTER_KEKY_USERNAME_EXISTS = "register.keky.username.exists",
	REGISTER_KEKY_PASSWORD = "register.keky.password",
	REGISTER_KEKY_PASSWORD_REPEAT = "register.keky.password.repeat",
	REGISTER_KEKY_PASSWORD_WEAK = "register.keky.password.weak",
	REGISTER_KEKY_PASSWORD_SPACE = "register.keky.password.space",
	REGISTER_KEKY_PASSWORD_MATCH = "register.keky.password.match",
	// ERRORS
	ERROR_CONNECTION = "api.error.connection"
}
