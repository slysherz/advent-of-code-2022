use std::collections::VecDeque;
use std::fs;

fn parse_crates(crate_input: &str, num_crates: usize) -> Vec<VecDeque<char>> {
    let mut crates = vec![VecDeque::new(); num_crates];
    for line in crate_input.split('\n') {
        let chars = line.chars().collect::<Vec<char>>();
        for (n, chunk) in chars.chunks(4).enumerate() {
            let c = chunk[1];
            if c >= 'A' && c <= 'Z' {
                crates[n].push_front(c);
            }
        }
    }

    crates
}

fn get_top_row(crates: Vec<VecDeque<char>>) -> String {
    let mut result = String::from("");
    for mut cr in crates {
        let c: char = cr.pop_back().unwrap();
        result.push(c);
    }

    result
}

fn solve1(str: String) -> String {
    let mut inputs = str.split("\n\n");
    let first_line = str.split('\n').next().unwrap();
    let num_crates = (first_line.len() + 1) / 4;
    let crate_input = inputs.next().unwrap();
    let move_input = inputs.next().unwrap();

    let mut crates = parse_crates(crate_input, num_crates);

    for line in move_input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut tokens = line.split(' ');
        tokens.next();
        let num = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let from = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let to = tokens.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..num {
            let c = crates[from - 1].pop_back().unwrap();
            crates[to - 1].push_back(c);
        }
    }

    get_top_row(crates)
}

fn solve2(str: String) -> String {
    let mut inputs = str.split("\n\n");
    let first_line = str.split('\n').next().unwrap();
    let num_crates = (first_line.len() + 1) / 4;
    let crate_input = inputs.next().unwrap();
    let move_input = inputs.next().unwrap();

    let mut crates = parse_crates(crate_input, num_crates);

    for line in move_input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut tokens = line.split(' ');
        tokens.next();
        let num = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let from = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let to = tokens.next().unwrap().parse::<usize>().unwrap();

        let final_length = crates[from - 1].len() - num;
        let tail = crates[from - 1].split_off(final_length);

        for c in tail {
            crates[to - 1].push_back(c);
        }
    }

    get_top_row(crates)
}

#[test]
fn example1() {
    let result = solve1(String::from(
        "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    ));

    assert_eq!(result, "CMZ")
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    ));

    assert_eq!(result, "MCD")
}

fn main() {
    let input = fs::read_to_string("../input/day-5.txt").expect("Unable to read file");
    // println!("{}", solve1(input));
    println!("{}", solve2(input));
}
