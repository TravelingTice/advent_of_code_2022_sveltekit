<script lang="ts">
	import Part2 from './Part2.svelte'
	import Part1 from './Part1.svelte'

	export let data: { input: string }
</script>

<h2>Part 1</h2>
<Part1 input={data.input} />

<h2>Part 2</h2>
<Part2 input={data.input} />
