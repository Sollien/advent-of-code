const fs = require('fs')

function readLines(fileName, separator = "\n", mapLine = x => x) {
	return fs.readFileSync(fileName, "utf-8").trimEnd().split(separator).map(mapLine)
}

function findNumbers(stringValue) {
	return stringValue.matchAll(/\d+/g)
}

function sum(arr) {
	return arr.reduce((x, y) => x + y, 0)
}

module.exports = {
	readLines,
	findNumbers,
	log: console.log,
	sum
}