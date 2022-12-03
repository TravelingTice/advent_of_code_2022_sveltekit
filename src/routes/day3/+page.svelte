<script lang="ts">
	import sum from '$lib/sum';

	export let data: { input: string };

	const backpacks = data.input.split('\n').filter((backpack) => Boolean(backpack));

	const findCommonCharacter = (str1: string, str2: string) => {
		return str1.split('').find((char) => str2.match(new RegExp(char)));
	};

	const getErrorLetter = (backpack: string) => {
		const compartment1 = backpack.slice(0, backpack.length / 2);
		const compartment2 = backpack.slice(backpack.length / 2, backpack.length);

		return findCommonCharacter(compartment1, compartment2);
	};

	const getPriorityForLetter = (letter: string) => {
		const prioLetters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
		return prioLetters.indexOf(letter) + 1;
	};

	const prioList = backpacks.map((backpack) => {
		return getPriorityForLetter(getErrorLetter(backpack)!);
	});

	const answer = sum(prioList);
</script>

<p>{answer}</p>
