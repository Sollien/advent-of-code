const { readLines, log } = require("../common")
let input = readLines("./input.txt", "\n\n")

const seeds = input[0].split(": ")[1].split(" ").map(x => parseInt(x))
const maps = input.slice(1).map(x => x.split("\n").slice(1).map(y => y.split(" ").map(z => parseInt(z))))
const result = []

function translate(index, value) {
    if (index == maps.length) return value

    for (const [destination, source, range] of maps[index]) {
        if (source <= value && value < source + range) return translate(index + 1, destination + value - source)
    }

    return translate(index + 1, value)
}

for (const seed of seeds) {
    result.push(translate(0, seed))
}

log(Math.min(...result))