const getPriorityForLetter = (letter: string) => {
	const prioLetters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
	return prioLetters.indexOf(letter) + 1;
};

export default getPriorityForLetter;
