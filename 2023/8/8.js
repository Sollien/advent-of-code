const { readLines, log, sum } = require('../common')

const input = readLines('./input.txt')
const pattern = input[0].replace(/\r/,"")
const nodeArray = []

input.shift()

input.forEach(line => {
    //Clean up the lines
    line = line.replace(/["'()]/g,"").replace(/\r/,"").replaceAll(" ", "")

    const lineSplit = line.split(/=|,/)
    const node = {
        name: lineSplit[0],
        leftValue: lineSplit[1],
        rightValue: lineSplit[2]
    }

    nodeArray.push(node)
})

nodeArray.shift()
let currentMove = nodeArray.indexOf(nodeArray.find((node) => node.name === "AAA"))
let valueToFind = "AAA"
let counter = 0
let index = 0

while(valueToFind !== 'ZZZ') {
    const char = pattern[index]
    if (index === pattern.length) index = 0;
    if (char === "L") {
        valueToFind = nodeArray[currentMove].leftValue
    } else if (char === "R") {
        valueToFind = nodeArray[currentMove].rightValue
    }
    currentMove = nodeArray.indexOf(nodeArray.find((node) => node.name === valueToFind))
    index++
    counter++
    if (valueToFind === "ZZZ") return log(counter)
}