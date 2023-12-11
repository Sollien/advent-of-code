const { readLines, log, findNumbers, sum } = require("./../common")

const input = readLines("./input.txt")

const isAdjacentToSymbol = (input, x1, x2, row) => {
	for (let y = row - 1; y <= row + 1; y++) {
		for (let x = x1 - 1; x <= x2 + 1; x++) {
			if (!input[y] || !input[y][x]) continue
			const char = input[y][x]
			if (char != '.' && !(char >= '0' && char <= '9')) return true
		}
	}

	return false
}

function partOne() {
	let adjacentNumbers = []

	input.flatMap((x, y) => {
		const numbersFound = Array.from(findNumbers(x))
		numbersFound.flatMap(match => {
			const { 0: number, index: x } = match
			adjacentNumbers.push(isAdjacentToSymbol(input, x, x + number.length - 1, y) ? parseInt(number) : 0)
		})
	})

	log(sum(adjacentNumbers))
}

partOne()