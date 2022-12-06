<script lang="ts">
	import init, { add } from 'day-6-rust'
	import { onMount } from 'svelte'

	onMount(async () => {
		await init()

		console.log(add(1, 5))
	})
</script>

<h2>Day 6 in WebAssembly!</h2>
