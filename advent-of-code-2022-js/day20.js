const fs = require('fs')

// Some tests to make sure things work
check([
    [1, 2, 3, 4],
    [1, 2, 3, 4]
], 2)

check([
    [1, 2, 3],
    [2, 1, 3]
])

check([
    [2, 1, 3],
    [1, 3, 2]
])

check([
    [1, 2, 3],
    [2, 1, 3]
])

check([
    [3, 2, -1],
    [3, -1, 2]
], 2)

check([
    [1, -2, 3, 4],
    [1, 3, -2, 4]
], 1)

check([
    [1, -1, 2, 3],
    [1, 2, 3, -1]
], 1)

check([
    [1, 2, -3, 0, 3, 4, -2],    // 4    6
    [1, 2, -3, 4, 0, 3, -2],
], 5)

check([
    [1, 2, -3, 3, -2, 0, 4],    // 1    0
    [2, 1, -3, 3, -2, 0, 4],    // 2    1
    [1, -3, 2, 3, -2, 0, 4],    // -3   2
    [1, 2, 3, -2, -3, 0, 4],    // 3    3
    [1, 2, -2, -3, 0, 3, 4],    // -2   4
    [1, 2, -3, 0, 3, 4, -2],    // 0    5
    [1, 2, -3, 0, 3, 4, -2],    // 4    6
    [1, 2, -3, 4, 0, 3, -2],
])

// Should print 3
run([1, 2, -3, 3, -2, 0, 4])

const data = fs.readFileSync('input/day-20.txt', 'utf-8')
const values = data.split('\r\n').map(s => parseInt(s))
// Part A
run(values)
// Part B
run(values.map(v => v * 811589153), 10)

function check(expected, start = 0) {
    let value = expected.shift()
    let order = value.map((_, i) => i)
    for (let i = 0; i < expected.length; i++) {
        const j = i + start
        const startOrder = order
        order = mix(value, order, j)
        const got = get(value, order)
        if (!eq(expected[i], got)) {
            console.log(`FAILED STEP ${j} MOVING ${value[j]}\nFROM\t${get(value, startOrder)}\nEXP\t${expected[i]}\nGOT\t${got}`)
            console.assert(false)
        }
    }
    console.log('TEST PASSED')
}

function run(value, mixes = 1) {
    let order = value.map((_, i) => i)
    for (let j = 0; j < mixes; j++) {
        for (let i = 0; i < order.length; i++) {
            order = mix(value, order, i)
        }
    }

    const res = get(value, order)
    const z = res.indexOf(0)
    console.log(res[(z + 1000) % res.length] + res[(z + 2000) % res.length] + res[(z + 3000) % res.length])
}

function eq(a, b) {
    return JSON.stringify(a) === JSON.stringify(b)
}

function get(value, order) {
    return order.map(p => value[p])
}

function mix(value, order, step) {
    return shift(value[step], order, step)
}

function move(order, a, b) {
    const n = order[a]

    if (a < b) {
        const i = a
        const j = b
        const l = order.slice(0, i)
        const m = order.slice(i + 1, j + 1)
        const r = order.slice(j + 1)
        return [
            ...l,
            ...m,
            n,
            ...r
        ]
    } else {
        const i = b
        const j = a
        const l = order.slice(0, i)
        const m = order.slice(i, j)
        const r = order.slice(j + 1)
        return [
            ...l,
            n,
            ...m,
            ...r
        ]
    }
}

function shift(times = 0, order = [], step = 0) {
    if (times === 0) {
        return order;
    }

    const i = order.indexOf(step)
    let j =  i + times
    if (times > 0) {
        let j =  1 + (i + times - 1) % (order.length - 1)
        return move(order, i, j)
    } else {
        const a = i + times
        const b = order.length
        let j =  b - (1 + (((b - a) - 1) % (b - 1)))
        return move(order, i, j)
    }
}