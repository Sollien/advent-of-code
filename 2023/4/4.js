const fs = require('fs')
const input = fs.readFileSync("./input.txt", "utf-8")
const lines = input.split("\n")
let data = []
let sum = 0

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
		const numberOfWinnerNumbers = []
		const winnerNumbers = new Set(data[i].winnerNumbers)
		const playerNumbers = new Set(data[i].playerNumbers)

		const intersections = [...winnerNumbers].filter(number => playerNumbers.has(number))

		numberOfWinnerNumbers.push(intersections.length)

		for (let j = 0; j < numberOfWinnerNumbers.length; j++) {
			if (numberOfWinnerNumbers[j] === 0) {
				sum += 0
			} else {
				sum += Math.pow(2, numberOfWinnerNumbers[j] - 1)
			}
		}
	}
}

parseData()
sumWinnerNumbers()

console.log(sum)