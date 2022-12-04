<script lang="ts">
	import type { Range } from './stringToRange';
	import stringToRange from './stringToRange';

	export let input: string;

	const rangeArray = (range: Range) => {
		const size = range[1] - range[0] + 1;
		const startAt = range[0];
		return [...Array(size).keys()].map((i) => i + startAt);
	};

	const isOverlapping = (range1: Range, range2: Range) => {
		return rangeArray(range1).some((nr) => rangeArray(range2).includes(nr));
	};

	const rangePairs = input
		.split('\n')
		.filter((str) => Boolean(str))
		.map((str) => str.split(',').map((range) => stringToRange(range)));

	const answer = rangePairs.filter((pair) => isOverlapping(pair[0], pair[1])).length;
</script>

<p>{answer}</p>
