const fs = require('fs')

const input = `
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32`

const monkeys = parse(fs.readFileSync('./input/day-21.txt', 'utf-8'))
// const monkeys = parse(input)
solve2(monkeys)

function parseExpression(exp) {
    const num = exp.match(/[0-9]+/)
    if (num) {
        return BigInt(num[0])
    } else {
        const [_, left, op, right] = exp.match(/(\w+) (.) (\w+)/)
        return {left, op, right}
    }
}

function parse(input) {
    let result = {}
    for (const [line, name, exp] of input.matchAll(/(\w+): (.+)/g)) {
        result[name] = parseExpression(exp)
    }

    return result
}

function reduceable(monkeys, name) {
    const exp = monkeys[name]
    return typeof(exp) !== 'bigint'
    &&  typeof(monkeys[exp?.left]) === 'bigint'
    &&  typeof(monkeys[exp?.right]) === 'bigint'
}

function trim(monkeys) {
    while (true) {
        const reduceableNames = Object.keys(monkeys).filter(n => reduceable(monkeys, n))
        if (reduceableNames.length === 0) {
            break
        }

        reduceableNames.forEach(n => {
            monkeys[n] = solve(monkeys, n)
        })
    }

    console.log(Object.values(monkeys).filter(e => typeof(e) !== 'bigint').length, "non trimmed entries")
    return;
}

function solve(monkeys, name, cache = {}) {
    const e = monkeys[name]
    if (typeof(e) === 'bigint') {
        return e
    } 

    if (cache.hasOwnProperty(name)) {
        return cache[name]
    }

    const {left, right} = e
    const l = solve(monkeys, left, cache)
    const r = solve(monkeys, right, cache)
    
    let result
    switch (e.op) {
        case '+': {
            result = l + r;
            break;
        }
        case '-': {
            result = l - r;
            break;
        }
        case '/': {
            result = l / r;
            break;
        }
        case '*': {
            result = l * r;
            break;
        }
    }
    
    // console.assert(result < Number.MAX_SAFE_INTEGER && result > Number.MIN_SAFE_INTEGER)
    cache[name] = result
    return result
}

function resolve(monkeys, nameOrValue) {
    return typeof(nameOrValue) === 'string' ? monkeys[nameOrValue] : nameOrValue
}

function solveEq(monkeys, left, right) {
    if (typeof(right) === 'bigint') {
        return solveEq(monkeys, right, left)
    }

    if (typeof(right) === 'object' && right === null) {
        return left
    }

    console.assert(typeof(left) === 'bigint' && typeof(right) === 'object')
    const rl = resolve(monkeys, right.left)
    const rr = resolve(monkeys, right.right)
    if (right.op === '+') {
        if (typeof(rl) === 'bigint') {
            return solveEq(monkeys, left - rl, rr)
        } else {
            return solveEq(monkeys, left - rr, rl)
        }
    }
    if (right.op === '*') {
        if (typeof(rl) === 'bigint') {
            return solveEq(monkeys, left / rl, rr)
        } else {
            return solveEq(monkeys, left / rr, rl)
        }
    }
    if (right.op === '-') {
        if (typeof(rl) === 'bigint') {
            return solveEq(monkeys, rl -  left, rr)
        } else {
            return solveEq(monkeys, left + rr, rl)
        }
    }
    if (right.op === '/') {
        if (typeof(rl) === 'bigint') {
            return solveEq(monkeys, rl / left, rr)
        } else {
            return solveEq(monkeys, left * rr, rl)
        }
    }

    console.assert(false)
}

function solve2(monkeys) {
    monkeys['humn'] = null
    trim(monkeys)
    const {left, right} = monkeys['root']
    console.log(solveEq(monkeys, monkeys[left], monkeys[right]))
}