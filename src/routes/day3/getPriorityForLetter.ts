const getPriorityForLetter = (letter: string) => {
	const prioLetters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
	return prioLetters.indexOf(letter) + 1; // Props to Jochem for giving the tip of adding the +1 here, didn't know of any context but was able to find out the bug and solve the puzzle instantly 🕶
};

export default getPriorityForLetter;
