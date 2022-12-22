<script lang="ts">
	import sum from '$lib/sum'
	import { onMount } from 'svelte'
	import Folder from './Folder'

	export let data: { input: string }

	let answer1: number
	let answer2: number

	const { input } = data

	onMount(async () => {
		const rootFolder = new Folder('root')

		// This folder will be the "state" of current folder to add contents to
		let currentFolder = rootFolder

		const commands = input.trim().split('$')

		commands.forEach((command) => {
			if (!command.trim()) return

			const [_, action, input] = command.trim().match(/^(cd|ls)[\s|\n]([\s\S]*)/)!

			if (action === 'ls') {
				currentFolder.addContents(input)
			} else {
				if (input === '/') currentFolder = rootFolder
				else currentFolder = currentFolder.findChild(input)
			}
		})

		const allFolders = rootFolder.getAllChildFolders()

		// For answer 1: Get the sum of all the folders with a size lesser than 100000
		const foldersOfSmallSize = allFolders.filter((folder) => folder.size() <= 100000)

		answer1 = sum(foldersOfSmallSize.map((folder) => folder.size()))

		// For answer 2: Find the smallest folder with a size that clears up 30000000 on the disk

		const totalDiskSize = 70000000
		const totalSizeToFreeUp = 30000000
		const currentSize = rootFolder.size()

		const currentFreeSize = totalDiskSize - currentSize

		const sizeToFreeUp = totalSizeToFreeUp - currentFreeSize

		console.log(sizeToFreeUp)

		const foldersOfBigEnoughSize = allFolders.filter((folder) => folder.size() >= sizeToFreeUp)
		const sortedFoldersOfBigEnoughSize = foldersOfBigEnoughSize.sort((a: Folder, b: Folder) =>
			a.size() > b.size() ? 1 : -1
		)

		answer2 = sortedFoldersOfBigEnoughSize[0].size()
	})
</script>

<h2>Day 7 😳</h2>

{#if !answer1 && !answer2}
	<p>Loading the answer...</p>
{:else}
	<p>Answer 1: {answer1}</p>
	<p>Answer 2: {answer2}</p>
{/if}
