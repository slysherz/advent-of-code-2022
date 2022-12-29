use std::{cmp::Ordering, fs};

enum Signal {
    Num(usize),
    List(Vec<Signal>),
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Signal::Num(l), Signal::Num(r)) => {
                if l < r {
                    Ordering::Less
                } else if l > r {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Signal::List(l), Signal::List(r)) => {
                let mut i = 0;
                let limit = usize::min(l.len(), r.len());

                while i < limit {
                    let res = l[i].cmp(&r[i]);
                    if res != Ordering::Equal {
                        return res;
                    }

                    i += 1;
                }

                if l.len() < r.len() {
                    Ordering::Less
                } else if l.len() > r.len() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (l, Signal::Num(r)) => l.cmp(&Signal::List(vec![Signal::Num(*r)])),
            (Signal::Num(l), r) => Signal::List(vec![Signal::Num(*l)]).cmp(r),
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Signal {}

fn parse_signal(str: &str) -> (Signal, usize) {
    let mut i = if str.chars().nth(0).unwrap() == ',' {
        1
    } else {
        0
    };
    let c1 = str.chars().nth(i).unwrap();
    if c1 == '[' {
        i += 1;
        let mut result = Vec::new();
        while str.chars().nth(i).unwrap() != ']' {
            let (s, j) = parse_signal(&str[i..]);
            result.push(s);
            i += j;
        }
        (Signal::List(result), i + 1)
    } else {
        let mut num = String::new();
        while let Some(c) = str.chars().nth(i) {
            if c.is_digit(10) {
                num.push(c);
            } else {
                if c == ',' {
                    i += 1;
                }
                break;
            }

            i += 1;
        }

        (Signal::Num(num.parse::<usize>().unwrap()), i)
    }
}

fn solve1(str: String) -> usize {
    let mut i = 1;
    let mut sum = 0;
    for line in str.split("\n\n") {
        let mut it = line.split('\n');
        let (left, _) = parse_signal(it.next().unwrap().trim());
        let (right, _) = parse_signal(it.next().unwrap().trim());
        if left < right {
            sum += i;
        }

        i += 1;
    }

    sum
}

fn solve2(str: String) -> usize {
    let mut all: Vec<Signal> = Vec::new();
    let div1 = "[[2]]";
    let div2 = "[[6]]";
    all.push(parse_signal(div1).0);
    all.push(parse_signal(div2).0);

    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        all.push(parse_signal(line).0);
    }

    all.sort();

    let f = all.iter().position(|s| s == &parse_signal(div1).0).unwrap() + 1;
    let l = all.iter().position(|s| s == &parse_signal(div2).0).unwrap() + 1;

    f * l
}

fn test_fn(a: &str, b: &str, res: bool) {
    let order = if res {
        Ordering::Less
    } else {
        Ordering::Greater
    };
    assert_eq!(parse_signal(a).0.cmp(&parse_signal(b).0), order);
}

#[test]
fn example1() {
    test_fn("[1,1,3,1,1]", "[1,1,5,1,1]", true);
    test_fn("[[1],[2,3,4]]", "[[1],4]", true);
    test_fn("[9]", "[[8,7,6]]", false);
    test_fn("[[4,4],4,4]", "[[4,4],4,4,4]", true);
    test_fn("[7,7,7,7]", "[7,7,7]", false);
    test_fn("[]", "[3]", true);
    test_fn("[[[]]]", "[[]]", false);
    test_fn(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        false,
    );

    let result = solve1(String::from(
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ));
    assert_eq!(result, 13);
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]",
    ));

    assert_eq!(result, 140);
}

fn main() {
    let input = fs::read_to_string("../input/day-13.txt").expect("Unable to read file");
    // println!("\n{}", solve1(input));
    println!("\n{}", solve2(input));
}
