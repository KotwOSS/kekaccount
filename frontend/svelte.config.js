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
			},
            // build: {
            //     rollupOptions: {
            //         plugins: [
            //             legacy({
            //                 targets: ['ie >= 11'],
            //                 additionalLegacyPolyfills: ['regenerator-runtime/runtime']
            //             })
            //         ]
            //     }
            // },
            // plugins: [
            //     // legacy({
            //     //     targets: ['ie >= 11'],
            //     //     additionalLegacyPolyfills: ['regenerator-runtime/runtime']
            //     // })
            // ]
		},
		trailingSlash: "always"
	}
};

export default config;
