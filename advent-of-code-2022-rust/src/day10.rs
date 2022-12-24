use std::fs;

fn signal_strength(mut cycle: i32, reg: i32, result: i32) -> (i32, i32) {
    let pixel = cycle % 40;
    let is_lit = reg - 1 <= pixel && reg + 1 >= pixel;
    if cycle % 40 == 0 {
        println!();
    }
    print!("{}", if is_lit { "#" } else { " " });

    cycle += 1;
    (
        cycle,
        result + if cycle % 40 == 20 { cycle * reg } else { 0 },
    )
}

fn solve(str: String) -> i32 {
    let mut reg = 1;
    let mut cycle = 0;
    let mut result = 0;
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        if line == "noop" {
            (cycle, result) = signal_strength(cycle, reg, result);
        } else {
            (cycle, result) = signal_strength(cycle, reg, result);
            (cycle, result) = signal_strength(cycle, reg, result);
            reg += line[5..].to_string().parse::<i32>().unwrap();
        }
    }

    result
}

#[test]
fn example1() {
    let result = solve(String::from(
        "
noop
addx 3
addx -5",
    ));

    assert_eq!(result, 0);
}

#[test]
fn example2() {
    let result = solve(String::from(
        "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    ));

    assert_eq!(result, 13140);
}

fn main() {
    let input = fs::read_to_string("../input/day-10.txt").expect("Unable to read file");
    println!("\n{}", solve(input));
}
