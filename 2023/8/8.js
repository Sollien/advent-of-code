const { readLines, log, sum } = require('../common')

const input = readLines('./input.txt')
const pattern = input[0].replace(/L/g, "0").replace(/R/g, "1").split("")
const nodeArray = []

function partOne() {
	let currentValue = "AAA"
	let valueToFind = "ZZZ"
	let counter = 0

	for (const line of input.slice(2)) {
		const node = line.split(" = ")
		nodeArray[node[0]] = JSON
			.parse(node[1]
			.replace("(", "[\"")
			.replace(")", "\"]")
			.replace(", ", "\", \""))
	}

	while(currentValue !== valueToFind) {
		for (const iteration of pattern) {
			currentValue = nodeArray[currentValue][iteration];
			counter++
			if (currentValue === valueToFind) return log(counter)
		}
	}
}

partOne()