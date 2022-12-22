<script lang="ts">
	import init, { test } from 'local-webassembly'
	import { onMount } from 'svelte'

	export let data: { input: string }

	onMount(async () => {
		await init()

		test()
	})
</script>

<h2>Day 7 😳</h2>
