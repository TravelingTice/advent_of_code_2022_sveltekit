<style>
	:global(body) {
		font-family: Roboto, sans-serif;
		margin: 0;
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	:global(h1, h2, h3, h4, h5, h6) {
		font-family: 'Open Sans', sans-serif;
	}

	:global(ul) {
		list-style: none;
		padding: 0;
	}

	:global(a) {
		color: #007991;
		text-decoration: none;
	}

	:global(button) {
		background-color: #454955;
		color: white;
		font-weight: bold;
		padding: 10px 20px;
		border-radius: 4px;
		border: none;
		cursor: pointer;
		transition: background-color 150ms ease-out;
	}

	:global(button:hover) {
		background-color: #33363e;
	}

	:global(button:disabled) {
		background-color: #797f8f;
	}
</style>
