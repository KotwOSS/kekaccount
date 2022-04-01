import { dev } from "$app/env";

export async function handle({ event, resolve }) {
	return await resolve(event, {
		ssr: !dev
	});
}
