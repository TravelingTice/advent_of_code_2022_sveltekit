export type Range = [number, number]

const stringToRange = (range: string): Range => {
	const rangeArr = range.split('-')
	return [Number(rangeArr[0]), Number(rangeArr[1])]
}

export default stringToRange
