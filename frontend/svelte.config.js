import adapter from "@sveltejs/adapter-static";
import preprocess from "svelte-preprocess";
import path from "path";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
			fallback: "404.html"
		}),
		prerender: {
			default: true
		},
		vite: {
			resolve: {
				alias: {
					$lib: path.resolve("src/lib"),
					$components: path.resolve("./src/components")
				}
			}
		},
		trailingSlash: "always"
	}
};

export default config;
