<div id="loading">
	<svg viewBox="0 0 100 100">
		<defs>
			<filter id="shadow">
				<feDropShadow dx="0" dy="0" stdDeviation="1.5" flood-color="#ffffff" />
			</filter>
		</defs>
		<circle
			id="spinner"
			style="fill:transparent;stroke:#ffffff;stroke-width: 7px;stroke-linecap: round;filter:url(#shadow);"
			cx="50"
			cy="50"
			r="45"
		/>
	</svg>
</div>

<style>
	@keyframes loader {
		0% {
			stroke-dasharray: 1 98;
			stroke-dashoffset: -105;
		}
		50% {
			stroke-dasharray: 80 10;
			stroke-dashoffset: -160;
		}
		100% {
			stroke-dasharray: 1 98;
			stroke-dashoffset: -300;
		}
	}

	#spinner {
		transform-origin: center;
		animation-name: loader;
		animation-duration: 1.2s;
		animation-timing-function: cubic-bezier;
		animation-iteration-count: infinite;
	}
</style>
