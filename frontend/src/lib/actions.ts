export type Action = {
	warning: string | undefined;
	hint: string | undefined;
	url: string;
};

export default [
	// Delete account
	{
		warning: "This will delete your account!",
		url: "/dash/account?d=1"
	},
	// Delete token
	{
		warning: "This will delete a token",
		url: "/dash/tokens?d=1"
	},
	// Verify
	{
		hint: "This will verify your account",
		url: "/verify?d=1"
	}
] as Action[];
