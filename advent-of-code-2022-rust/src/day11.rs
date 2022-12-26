use std::fs;

#[derive(Clone, Copy)]
enum Op {
    Plus,
    Times,
}

#[derive(Clone, Copy)]
enum Arg {
    Old,
    Number(usize),
}

type Operation = (Op, Arg);

struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
}

fn parse_items(str: &str) -> Vec<usize> {
    str[18..]
        .split(", ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn parse_operation(str: &str) -> Operation {
    let last_token = str.split(' ').last().unwrap();
    (
        match str.chars().nth(23).unwrap() {
            '*' => Op::Times,
            '+' => Op::Plus,
            _ => std::unreachable!(),
        },
        if last_token == "old" {
            Arg::Old
        } else {
            Arg::Number(parse_last_int(last_token))
        },
    )
}

fn parse_last_int(str: &str) -> usize {
    str.split(' ').last().unwrap().parse().unwrap()
}

fn parse_monkey(str: &str) -> Monkey {
    let lines: Vec<&str> = str.split('\n').collect();
    if let [id, items, operation, test, if_true, if_false] = lines[..] {
        let len = id.len() - 1;
        return Monkey {
            id: parse_last_int(&id[0..len]),
            items: parse_items(items),
            operation: parse_operation(operation),
            test: parse_last_int(test),
            if_true: parse_last_int(if_true),
            if_false: parse_last_int(if_false),
        };
    }

    std::unreachable!()
}

fn solve1(str: String) -> usize {
    let mut monkeys = Vec::new();

    for monkey in str.split("\n\n") {
        monkeys.push(parse_monkey(monkey.trim()))
    }

    let mut inspects: Vec<usize> = monkeys.iter().map(|_| 0).collect();
    let rounds = 20;

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            for item in monkeys[m].items.clone() {
                let new_item = match monkeys[m].operation {
                    (Op::Plus, Arg::Old) => item + item,
                    (Op::Times, Arg::Old) => item * item,
                    (Op::Plus, Arg::Number(n)) => item + n,
                    (Op::Times, Arg::Number(n)) => item * n,
                } / 3;

                let to = if new_item % monkeys[m].test == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys.get_mut(to).unwrap().items.push(new_item);

                inspects[monkeys[m].id] += 1;
            }
            monkeys.get_mut(m).unwrap().items.clear();
        }
    }

    inspects.sort();
    inspects.iter().rev().take(2).product()
}

fn solve2(str: String) -> usize {
    let mut monkeys = Vec::new();

    for monkey in str.split("\n\n") {
        monkeys.push(parse_monkey(monkey.trim()))
    }

    let mut mmc = 1;
    for monkey in &monkeys {
        mmc *= monkey.test;
    }

    let mut inspects: Vec<usize> = monkeys.iter().map(|_| 0).collect();
    let rounds = 10000;

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            for item in monkeys[m].items.clone() {
                let new_item = match monkeys[m].operation {
                    (Op::Plus, Arg::Old) => item + item,
                    (Op::Times, Arg::Old) => item * item,
                    (Op::Plus, Arg::Number(n)) => item + n,
                    (Op::Times, Arg::Number(n)) => item * n,
                } % mmc;

                let to = if new_item % monkeys[m].test == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys.get_mut(to).unwrap().items.push(new_item);

                inspects[monkeys[m].id] += 1;
            }
            monkeys.get_mut(m).unwrap().items.clear();
        }
    }

    inspects.sort();
    inspects.iter().rev().take(2).product()
}

#[test]
fn example1() {
    let result = solve1(String::from(
        "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
    ));

    assert_eq!(result, 10605)
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
    ));

    assert_eq!(result, 2713310158)
}

fn main() {
    let input = fs::read_to_string("../input/day-11.txt").expect("Unable to read file");
    // println!("\n{}", solve1(input));
    println!("\n{}", solve2(input));
}
