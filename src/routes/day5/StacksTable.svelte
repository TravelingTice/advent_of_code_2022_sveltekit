<script lang="ts">
	import type { Stacks } from './proxy+page'

	export let currentStack: Stacks

	const highestStackLength = (stacks: Stacks) => {
		const stackLengths = Object.keys(stacks).map((stack) => stacks[stack].length)
		return Math.max(...stackLengths)
	}
</script>

<table>
	<tbody>
		{#each [...Array(highestStackLength(currentStack)).keys()] as stackHeight}
			<tr>
				{#each Object.keys(currentStack) as stackNr}
					<td>{currentStack[stackNr][highestStackLength(currentStack) - stackHeight - 1] ?? ''}</td>
				{/each}
			</tr>
		{/each}
	</tbody>
	<tfoot>
		<tr>
			{#each Object.keys(currentStack) as stackNr}
				<td><strong>{stackNr}</strong></td>
			{/each}
		</tr>
	</tfoot>
</table>

<style>
	td {
		width: 30px;
	}
	tfoot td {
		padding-top: 10px;
	}
</style>
