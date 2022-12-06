<script lang="ts">
	import init, { get_start_packet_marker, get_start_message_marker } from 'day-6-rust'
	import { onMount } from 'svelte'

	export let data: { message: string }

	export let packetMarker: number = 0
	export let messageMarker: number = 0

	onMount(async () => {
		await init()

		packetMarker = get_start_packet_marker(data.message)
		messageMarker = get_start_message_marker(data.message)
	})
</script>

<h2>Day 6 in WebAssembly!</h2>

{#if packetMarker && messageMarker}
	<p>The marker at which a packet has arrived is at: {packetMarker}</p>
	<p>And it's message arrives at marker: {messageMarker}</p>
{:else}
	<p>Doing the calculation in WebAssembly so quickly you won't even be able to see this!</p>
{/if}

<p>
	<a
		target="_blank"
		href="https://github.com/TravelingTice/advent_of_code_2022_sveltekit/blob/master/src/routes/day6"
		>See the code for this day</a
	>
</p>
