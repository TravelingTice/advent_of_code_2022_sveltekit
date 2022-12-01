<script>
	import GlobalStyles from '$lib/GlobalStyles.svelte';
	import Nav from '$lib/Nav.svelte';
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
	<link
		href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@600&family=Roboto&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<GlobalStyles />

<header class="header">
	<h2>Advent of code 2022 Tice Kralt 👉 SvelteKit</h2>
</header>

<header class="sub-header">
	<Nav />
</header>

<main>
	<slot />
</main>

<footer>Go to ticekralt.com</footer>

<style>
	.header {
		color: white;
		padding: 12px;
		background-color: #223843;
	}

	.sub-header {
		background-color: #dbd3d8;
		color: #223843;
		padding: 12px;
	}

	main {
		flex-grow: 1;
		padding: 12px;
		color: #223843;
		margin: 0 auto;
		text-align: center;
	}

	@media (min-width: 600px) {
		main {
			width: 600px;
			text-align: left;
		}
	}

	footer {
		padding: 12px;
		color: white;
		background-color: #d77a61;
		text-align: center;
	}
</style>
