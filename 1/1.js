const fs = require('fs')
const inputList = fs.readFileSync("./input.txt", "utf-8").split("\n")
const sumList = []

for (let i = 0; i < inputList.length; i++) {
	const numbersInString = inputList[i].match(/\d/g)

	if (numbersInString) {
		const stringToInt = numbersInString.map(match => parseInt(match, 10))

		if (stringToInt.length === 1) {
			const concatenatedNumbers = parseInt(stringToInt[0].toString() + stringToInt[0].toString(), 10)

			sumList.push(concatenatedNumbers)
		} else {
			const firstNumber = stringToInt[0]
			const lastNumber = stringToInt[stringToInt.length - 1]
			const concatenatedNumbers = parseInt(firstNumber.toString() + lastNumber.toString(), 10)

			sumList.push(concatenatedNumbers)
		}
	}
}

const sum = sumList.reduce((accumulator, currentNumber) => accumulator + currentNumber, 0)

console.log(sum)