<script lang="ts">
	import sum from '$lib/sum';

	export let input: string;

	const formattedInput = input
		.split('\n')
		.filter((elem) => Boolean(elem))
		.map((elem) => elem.split(' ')) as Match[];

	type TheirHand = 'A' | 'B' | 'C';
	type MyHand = 'X' | 'Y' | 'Z';

	type Match = [TheirHand, MyHand];

	const hasWonMatch = (match: Match): 'won' | 'draw' | 'lost' => {
		if (match[0] === 'A') {
			if (match[1] === 'X') return 'draw';
			if (match[1] === 'Y') return 'won';
			return 'lost';
		}

		if (match[0] === 'B') {
			if (match[1] === 'X') return 'lost';
			if (match[1] === 'Y') return 'draw';
			return 'won';
		}

		if (match[1] === 'X') return 'won';
		if (match[1] === 'Y') return 'lost';
		return 'draw';
	};

	const pointsForMyHand = (hand: MyHand): number => {
		if (hand === 'X') return 1;
		if (hand === 'Y') return 2;
		return 3;
	};

	const calcMatchResult = (match: Match): number => {
		let score = 0;
		const hasWon = hasWonMatch(match);
		if (hasWon === 'won') score += 6;
		if (hasWon === 'draw') score += 3;

		score += pointsForMyHand(match[1]);

		return score;
	};

	const scores = formattedInput.map((match) => {
		return calcMatchResult(match);
	});

	const answer = sum(scores);
</script>

<p>{answer}</p>
