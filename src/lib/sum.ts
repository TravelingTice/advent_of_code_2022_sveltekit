const sum = (arr: number[]) => {
	return arr.reduce((partSum, item) => partSum + item, 0)
}

export default sum
