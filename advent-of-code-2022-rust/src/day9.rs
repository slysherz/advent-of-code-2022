use std::fs;

fn next(head: i32, tail: i32) -> i32 {
    if head > tail {
        tail + 1
    } else if head < tail {
        tail - 1
    } else {
        tail
    }
}

fn next_pos((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let touching = i32::abs(hx - tx) <= 1 && i32::abs(hy - ty) <= 1;
    if touching {
        return (tx, ty);
    }

    (next(hx, tx), next(hy, ty))
}

fn solve1(str: String) -> usize {
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        let mut it = line.split(' ');
        let c = it.next().unwrap().chars().next().unwrap();
        let n = it.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..n {
            (hx, hy) = match c {
                'U' => (hx, hy + 1),
                'D' => (hx, hy - 1),
                'R' => (hx + 1, hy),
                'L' => (hx - 1, hy),
                _ => std::unreachable!(),
            };

            (tx, ty) = next_pos((hx, hy), (tx, ty));
            visited.insert((tx, ty));
        }
    }

    visited.len()
}

fn solve2(str: String) -> usize {
    let mut rope = vec![(0, 0); 10];
    let mut visited = std::collections::HashSet::new();
    visited.insert((0, 0));
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        let mut it = line.split(' ');
        let c = it.next().unwrap().chars().next().unwrap();
        let n = it.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..n {
            let (hx, hy) = rope[0];
            rope[0] = match c {
                'U' => (hx, hy + 1),
                'D' => (hx, hy - 1),
                'R' => (hx + 1, hy),
                'L' => (hx - 1, hy),
                _ => std::unreachable!(),
            };

            for i in 1..10 {
                rope[i] = next_pos(rope[i - 1], rope[i]);
            }
            visited.insert(rope[9]);
        }
    }

    visited.len()
}

#[test]
fn example1() {
    let result = solve1(String::from(
        "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    ));

    assert_eq!(result, 13);
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    ));

    assert_eq!(result, 1);
}

#[test]
fn example3() {
    let result = solve2(String::from(
        "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    ));

    assert_eq!(result, 36);
}

fn main() {
    let input = fs::read_to_string("../input/day-9.txt").expect("Unable to read file");
    // println!("{}", solve1(input));
    println!("{}", solve2(input));
}
