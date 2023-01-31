<script>
	import "../app.css";
</script>

<header>
	<nav class="flex justify-evenly bg-slate-900 text-white">
		<ul>
			<li>
				<a class="hover:text-cyan-400" href="/">Browse</a>
			</li>
			<li>
				<a class="hover:text-cyan-400" href="/edit">Edit</a>
			</li>
			<li>
				<a class="hover:text-cyan-400" href="/settings">Settings</a>
			</li>
			<li>
				<a class="hover:text-cyan-400" href="/source">Source</a>
			</li>
		</ul>
	</nav>
</header>

<style>
	
	ul {
		padding: 0;
		margin: 0;
		height: 3em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background: var(--background);
		background-size: contain;
	}



	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0.5rem;
		font-weight: 700;
		font-size: 0.8rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		text-decoration: none;
		transition: color 0.2s linear;
	}
</style>
