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
        const isEdgeRight = (num[1] + num[2]) % lineLength === 0
        
        let checkPositions = [
            ...numberRange(num[1] - (!isEdgeLeft && 1) - lineLength, num[1] + num[2] - lineLength + (!isEdgeRight && 1)),
            !isEdgeLeft && num[1] - 1,
            !isEdgeRight && num[1] + num[2],
            ...numberRange(num[1] - (!isEdgeLeft && 1) - lineLength, num[1] + num[2] - lineLength + (!isEdgeRight && 1)),
        ]

        checkPositions = checkPositions.filter((position) => position >= 0 && position < input.length)
        const isPartNumber = checkPositions.some((position) => symbolPositions.includes(position))
        return isPartNumber ? tot + parseInt(num[0]) : tot
    })

    return console.log(sum)
}

partOne()