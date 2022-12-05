<script lang="ts">
	import getPercentage from '$lib/getPercentage'
	import { onMount } from 'svelte'
	import type { Stacks } from './+page'
	import type { Move } from './proxy+page'
	import StacksTable from './StacksTable.svelte'

	export let data: {
		input: { startingStacksAsArray: Stacks; movesArray: Move[]; moveTitlesArray: string[] }
	}

	const { movesArray, moveTitlesArray } = data.input

	let currentStack = { ...data.input.startingStacksAsArray }
	let currentMove = 0

	let showingSolution: 1 | 2 = 1

	const getAnswerFromStack = (stacks: Stacks) => {
		return Object.keys(stacks)
			.map((nr) => stacks[nr][stacks[nr].length - 1])
			.join('')
	}

	const switchSolution = () => {
		if (showingSolution === 1) showingSolution = 2
		else showingSolution = 1

		currentMove = 0
		currentStack = { ...data.input.startingStacksAsArray }

		performMove(true)
	}

	let timeout: NodeJS.Timeout

	const performMove = (shouldClearTimeout = false) => {
		if (shouldClearTimeout) clearTimeout(timeout)

		if (currentMove >= movesArray.length) return

		const move = movesArray[currentMove]
		const currentStackLength = currentStack[move.from].length

		// Get stack to move
		const stackToMove = currentStack[move.from].slice(currentStackLength - move.amnt)

		// Remove it from the current stack
		currentStack[move.from] = currentStack[move.from].slice(0, currentStackLength - move.amnt)

		// Depending on the solution flip it and add it onto the stack where it will move to
		const newStack = showingSolution === 1 ? stackToMove.reverse() : stackToMove
		currentStack[move.to] = currentStack[move.to].concat(newStack)

		currentMove++

		timeout = setTimeout(performMove, 20)
	}

	onMount(() => {
		setTimeout(performMove, 20)
	})
</script>

<h2>Day 5 let's go</h2>

<p>
	<a
		target="_blank"
		href="https://github.com/TravelingTice/advent_of_code_2022_sveltekit/blob/master/src/routes/day5"
		>See the code for this day</a
	>
</p>

<button on:click={switchSolution} disabled={showingSolution === 1}>
	Show{#if showingSolution === 1}ing{/if} solution 1
</button>

<button on:click={switchSolution} disabled={showingSolution === 2}>
	Show{#if showingSolution === 2}ing{/if} solution 2
</button>

<div class="move-info">
	<p>
		{#if currentMove === moveTitlesArray.length}
			All moves completed!
		{:else}
			Move being performed: {moveTitlesArray[currentMove]}
		{/if}
	</p>
	<p>
		({currentMove}/{moveTitlesArray.length}, {getPercentage(currentMove, moveTitlesArray.length)}%)
	</p>
</div>

<p>Current answer: {getAnswerFromStack(currentStack)}</p>

<StacksTable {currentStack} />

<style>
	.move-info {
		display: flex;
		justify-content: space-between;
	}
</style>
