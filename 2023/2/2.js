const fs = require('fs')
const input = fs.readFileSync("./input.txt", 'utf-8')

const maxPerColor = (sets, color) => {
	const parseColor = new RegExp(`\\d+(\\.\\d)* ${color}+`, 'g')
	return Math.max(...(sets.match(parseColor).map(match => match.split(" ")[0])))
}

function validGames() {
	const games = input.split("\n")

	for (let i = 0; i < games.length; i++) {
		if (maxPerColor(games[i], "red") > 12 || maxPerColor(games[i], "green") > 13 || maxPerColor(games[i], "blue") > 14) {
			delete games[i]
		}
	}

	return games.filter((str) => str !== '')
}

function part2() {
	const games = input.split("\n")
	let sum = 0

	for (let i = 0; i < games.length; i++) {
		sum += (maxPerColor(games[i], "red") * maxPerColor(games[i], "green") * maxPerColor(games[i], "blue"))
	}

	return sum
}

function part1() {
	const games = validGames()

	let sum = 0

	for (let i = 0; i < games.length; i++) {
		const [gameIndex] = games[i].match(/Game \d+/)
		const id = parseInt(gameIndex.match(/\d+/)[0], 10)

		sum += id
	}

	return sum
}

console.log(part1())
console.log(part2())