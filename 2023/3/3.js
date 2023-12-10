const fs = require('fs')
let input = fs.readFileSync("./input.txt", "utf-8")

function numberRange(start, end) {
    return new Array(end - start).fill().map((d, i) => i + start)
}

function partOne() {
    const lineLength = input.indexOf("\n") + 1

    const symbolRegexp = /[^0-9.\n]/g
    const symbolPositions = [...input.matchAll(symbolRegexp)].map(match => match.index)

    const numbersRegexp = /[0-9]+/g
    const numbers = [...input.matchAll(numbersRegexp)].map(match => [match[0], match.index, match[0].length])

    const sum = numbers.reduce((tot, num) => {
        const isEdgeLeft = num[1] % lineLength === 0
        console.log(num[1] % lineLength)
    })
}

partOne()