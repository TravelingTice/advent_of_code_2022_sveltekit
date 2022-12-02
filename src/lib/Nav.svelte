<script lang="ts">
	const modules = import.meta.glob('../routes/**/*.svelte');
	const routes = Object.keys(modules)
		.filter((route) => route.match(/routes\/[^\+].+\+page.svelte$/))
		.map((route) => route.split('/')[2]);
</script>

<nav>
	<ul>
		<li><a href={`/`}>Home</a></li>
		{#each routes as route (route)}
			<li><a href={`/${route}`}>{route}</a></li>
		{/each}
	</ul>
</nav>

<style>
	ul {
		display: flex;
		justify-content: center;
	}

	li {
		margin-right: 12px;
	}
</style>
