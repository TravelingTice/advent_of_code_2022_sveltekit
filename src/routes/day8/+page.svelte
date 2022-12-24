<script lang="ts">
	import { onDestroy, onMount } from 'svelte'
	import fullwidth from '../fullwidthstore'

	export let data: { input: string[][] }

	interface Tree {
		height: number
		isVisible: boolean | null
	}

	let field: Tree[][] = []

	onMount(() => {
		// Convert input to field
		field = data.input.map((line) =>
			line.map((num) => ({
				height: Number(num),
				isVisible: null
			}))
		)

		doVisibilityCheckFromLeft()
		doVisibilityCheckFromRight()
		doVisibilityCheckFromTop()
		doVisibilityCheckFromBottom()
	})

	const doVisibilityCheckForLine = (line: Tree[]): Tree[] => {
		const newLineArr = [...line]
		let highestNumber = -1

		newLineArr.forEach((tree, i) => {
			if (tree.height > highestNumber) {
				newLineArr[i] = { ...tree, isVisible: true }
				highestNumber = tree.height
			}
		})

		return newLineArr
	}

	const doVisibilityCheckFromLeft = () => {
		field = field.map((line) => doVisibilityCheckForLine(line))
	}

	const reverse = (field: Tree[][]) => {
		return field.map((line) => {
			const clonedLine = [...line]
			clonedLine.reverse()
			return clonedLine
		})
	}

	const doVisibilityCheckFromRight = () => {
		let reversedField = reverse(field)
		reversedField = reversedField.map((line) => doVisibilityCheckForLine(line))
		// Reverse it back 🥴
		reversedField = reverse(reversedField)
		field = reversedField
	}

	const rotate = (field: Tree[][]) => {
		return field.map((line, i) => line.map((tree, j) => field[j][i]))
	}

	const doVisibilityCheckFromTop = () => {
		let rotatedField = rotate(field)
		rotatedField = rotatedField.map((line) => doVisibilityCheckForLine(line))
		// Rotate it back 🥴
		rotatedField = rotate(rotatedField)
		field = rotatedField
	}

	const doVisibilityCheckFromBottom = () => {
		let rotatedReversedField = rotate(field)
		rotatedReversedField = reverse(rotatedReversedField)
		rotatedReversedField = rotatedReversedField.map((line) => doVisibilityCheckForLine(line))
		// Rotate and reverse back 🥴
		rotatedReversedField = reverse(rotatedReversedField)
		rotatedReversedField = rotate(rotatedReversedField)
		field = rotatedReversedField
	}

	$: amntOfVisibleTrees = field.flat().filter((tree) => tree.isVisible).length

	onMount(fullwidth.on)
	onDestroy(fullwidth.off)
</script>

<h2>Day 8 let's gooooo</h2>

<p>Amount of visible trees: {amntOfVisibleTrees}</p>

<table>
	{#each field as line}
		<tr>
			{#each line as tree}
				<td class:visible={tree.isVisible}>{tree.height}</td>
			{/each}
		</tr>
	{/each}
</table>

<style>
	td {
		font-size: 11px;
	}

	td.visible {
		background-color: lightgreen;
	}
</style>
