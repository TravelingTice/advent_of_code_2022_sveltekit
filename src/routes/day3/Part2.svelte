<script lang="ts">
	import sum from '$lib/sum'
	import getPriorityForLetter from './getPriorityForLetter'

	export let input: string

	const elves = input.split('\n').filter((backpack) => Boolean(backpack))

	const elveTeams: string[][] = [[]]

	elves.forEach((elve) => {
		if (elveTeams[elveTeams.length - 1]?.length === 3) {
			elveTeams.push([elve])
		} else {
			elveTeams[elveTeams.length - 1].push(elve)
		}
	})

	type Team = [string, string, string]

	const findCommonCharacter = (team: Team): string => {
		return team[0].split('').find((item) => {
			return team[1].match(new RegExp(item)) && team[2].match(new RegExp(item))
		})!
	}

	const prioList = elveTeams.map((team) => getPriorityForLetter(findCommonCharacter(team as Team)))

	const answer = sum(prioList)
</script>

<p>{answer}</p>
