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

function partOne() {
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

	return console.log(winnerDistances.reduce((a, b) => a * b))
}

function partTwo() {
	let joinTime = ""
	let joinWinnerDistance = ""
	const winnerDistances = []

	races.forEach(race => {
		joinTime += race.time
		joinWinnerDistance += race.winnerDistance
	})

	const timeToInt = parseInt(joinTime, 10)
	const winnerDistanceToInt = parseInt(joinWinnerDistance, 10)

	for (let i = 0; i < timeToInt; i++) {
		const distance = (timeToInt - i) * i

		if (distance > winnerDistanceToInt) {
			winnerDistances.push(distance)
		}
	}

	console.log(winnerDistances.length)
}

partOne()
partTwo()