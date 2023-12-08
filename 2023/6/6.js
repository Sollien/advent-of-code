const races = [
	{
		time: 49,
		winnerDistance: 263,
		newWinnerDistances: []
	},
	{
		time: 97,
		winnerDistance: 1532,
		newWinnerDistances: []
	},
	{
		time: 94,
		winnerDistance: 1378,
		newWinnerDistances: []
	},
	{
		time: 94,
		winnerDistance: 1851,
		newWinnerDistances: []
	}
]

const winnerDistances = []

races.forEach(race => {
	for (let i = 0; i < race.time; i++) {
		const distance = (race.time - i) * i

		if (distance > race.winnerDistance) {
			race.newWinnerDistances.push(distance)
		}

	}

	winnerDistances.push(race.newWinnerDistances.length)
})

function partOneAnswer() {
	return console.log(winnerDistances.reduce((a, b) => a * b))
}

partOneAnswer()