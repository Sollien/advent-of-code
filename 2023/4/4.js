const fs = require('fs')
const { type } = require('os')
const input = fs.readFileSync("./input.txt", "utf-8")
const lines = input.split("\n")

let data = []
let part1Sum = 0

function parseData() {
	lines.forEach(line => {
		const words = line.split(/\s+/)

		const cardId = parseInt(words[1].slice(0, -1))
		const winnerNumbers = words.slice(2, 12).map(Number)
		const playerNumbers = words.slice(13).map(Number)

		const cardData = { id: cardId, winnerNumbers: winnerNumbers, playerNumbers: playerNumbers }
		data.push(cardData)
	})
}

function sumWinnerNumbers() {
	for (let i = 0; i < data.length; i++) {
		let numberOfWinnerNumbers = []
		const winnerNumbers = data[i].winnerNumbers
		const playerNumbers = data[i].playerNumbers

		const intersections = winnerNumbers.filter(number => playerNumbers.includes(number)).length

		numberOfWinnerNumbers.push(intersections)

		for (let j = 0; j < numberOfWinnerNumbers.length; j++) {
			const cardToCopy = numberOfWinnerNumbers[j]

			if (numberOfWinnerNumbers[j] === 0) {
				part1Sum += 0
			} else {
				part1Sum += Math.pow(2, numberOfWinnerNumbers[j] - 1)
			}
		}
	}

	return console.log(part1Sum)
}

parseData()
sumWinnerNumbers()