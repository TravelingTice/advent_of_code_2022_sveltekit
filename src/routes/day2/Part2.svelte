<script lang="ts">
	import sum from '$lib/sum';

	export let input: string;

	const formattedInput = input
		.split('\n')
		.filter((elem) => Boolean(elem))
		.map((elem) => elem.split(' ')) as Match[];

	type TheirHand = 'A' | 'B' | 'C'; // Rock, Paper, Scissors
	type Outcome = 'X' | 'Y' | 'Z'; // Lose, Draw, Win

	type Match = [TheirHand, Outcome];

	const myHandInMatch = (match: Match): 'rock' | 'paper' | 'scissors' => {
		// Have to lose
		if (match[1] === 'X') {
			if (match[0] === 'A') return 'scissors';
			if (match[0] === 'B') return 'rock';
			return 'paper';
		}

		// Have to play draw
		if (match[1] === 'Y') {
			if (match[0] === 'A') return 'rock';
			if (match[0] === 'B') return 'paper';
			return 'scissors';
		}

		// Have to win
		if (match[0] === 'A') return 'paper';
		if (match[0] === 'B') return 'scissors';
		return 'rock';
	};

	const pointsForOutcome = (outcome: Outcome): number => {
		if (outcome === 'X') return 0;
		if (outcome === 'Y') return 3;
		return 6;
	};

	const calcMatchResult = (match: Match): number => {
		let score = 0;
		const myHand = myHandInMatch(match);

		if (myHand === 'rock') score += 1;
		if (myHand === 'paper') score += 2;
		if (myHand === 'scissors') score += 3;

		score += pointsForOutcome(match[1]);

		return score;
	};

	const scores = formattedInput.map((match) => {
		return calcMatchResult(match);
	});

	const answer = sum(scores);
</script>

<p>{answer}</p>

<p>
	<a
		target="_blank"
		href="https://github.com/TravelingTice/advent_of_code_2022_sveltekit/blob/master/src/routes/day2/Part2.svelte"
		>See the code for this part</a
	>
</p>
