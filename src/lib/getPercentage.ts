const getPercentage = (amnt: number, total: number) => {
	return Math.round((amnt / total) * 100)
}

export default getPercentage
