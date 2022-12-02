<script lang="ts">
	import Part1 from './Part1.svelte';
	import Part2 from './Part2.svelte';

	export let data: { input: string };
</script>

<h2>Part 1:</h2>
<Part1 input={data.input} />

<h2>Part 2:</h2>
<Part2 input={data.input} />

<p>
	<a
		target="_blank"
		href="https://github.com/TravelingTice/advent_of_code_2022_sveltekit/blob/master/src/routes/day2"
		>See the code for this day</a
	>
</p>