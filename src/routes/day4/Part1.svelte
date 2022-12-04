<script lang="ts">
	import type { Range } from './stringToRange';
	import stringToRange from './stringToRange';

	export let input: string;

	const isInRange = (range1: Range, range2: Range) => {
		return range1[0] >= range2[0] && range1[1] <= range2[1];
	};

	const isOneInRangeOfOther = (range1: Range, range2: Range) => {
		return isInRange(range1, range2) || isInRange(range2, range1);
	};

	const rangePairs = input
		.split('\n')
		.filter((str) => Boolean(str))
		.map((str) => str.split(',').map((range) => stringToRange(range)));

	const answer = rangePairs.filter((pair) => isOneInRangeOfOther(pair[0], pair[1])).length;
</script>

<p>{answer}</p>
