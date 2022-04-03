export let language: any;
export let initialized: number = 0;

export const supported: any = {
	en_us: "🇬🇧 English",
	de: "🇩🇪 Deutsch"
};
export const fallback: string = "en_us";

export function init() {
	return new Promise((resolve) => {
		if (initialized++ === 0) {
			let preference =
				localStorage.getItem("lang") || navigator.language.replace("-", "_").toLowerCase();
			let fallbacked = supported[preference] ? preference : fallback;

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
	NAV_ACCOUNT = "nav.account",
	NAV_LOGOUT = "nav.logout",
	// FOOTER
	FOOTER_PRIVACY = "footer.privacy",
	FOOTER_IMPRINT = "footer.imprint",
	FOOTER_NOTICE = "footer.notice",
	FOOTER_LANGUAGE = "footer.language",
	FOOTER_LEGAL = "footer.legal",
	FOOTER_EVENTS = "footer.events",
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
	// LOGIN
	LOGIN_TITLE = "login.title",
	LOGIN_DESCRIPTION = "login.description",
	LOGIN_UOE = "login.uoe",
	LOGIN_PASSWORD = "login.password",
	LOGIN_SUBMIT = "login.submit",
	LOGIN_REGISTER = "login.register",
	// DASHBOARD
	DASHBOARD_TITLE = "dashboard.title",
	// ERRORS
	ERROR_CONNECTION = "error.connection",
	ERROR_CREDENTIALS = "error.credentials",
	// HINTS
	HINT_VERIFY = "hint.verify",
	// ACCOUNT
	ACCOUNT_TITLE = "account.title",
	ACCOUNT_DESCRIPTION = "account.description",
    ACCOUNT_DELETE = "account.delete",
    ACCOUNT_DELETE_CONFIRM = "account.delete.confirm",
	// APPS
	APPS_TITLE = "apps.title",
	APPS_DESCRIPTION = "apps.description",
	APPS_CREATE = "apps.create",
	// TOKENS
	TOKENS_TITLE = "tokens.title",
	TOKENS_DESCRIPTION = "tokens.description",
	TOKENS_CREATE = "tokens.create",
    // CONFIRM
    CONFIRM_TITLE = "confirm.title",
    CONFIRM_DESCRIPTION = "confirm.description",
    CONFIRM_SUBMIT = "confirm.submit",
    CONFIRM_TYPE_TO_CONFIRM = "confirm.type_to_confirm",
    // VERIFY
    VERIFY_CONFIRM_HINT = "verify.confirm.hint",

	// MISC
	BACK_TO_DASHBOARD = "back.to.dashboard"
}
