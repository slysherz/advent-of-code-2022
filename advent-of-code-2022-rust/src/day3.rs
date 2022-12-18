use std::fs;

fn find_repeat(str: &str) -> char {
    let rucksack1 = &str[0..str.len() / 2];
    let rucksack2 = &str[str.len() / 2..str.len()];

    for c1 in rucksack1.chars() {
        for c2 in rucksack2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    ' '
}

fn find_repeat3(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    for c1 in rucksack1.chars() {
        for c2 in rucksack2.chars() {
            for c3 in rucksack3.chars() {
                if c1 == c2 && c1 == c3 {
                    return c1;
                }
            }
        }
    }

    ' '
}

fn priority(c: char) -> i32 {
    let code = c as i32;

    if code > 95 {
        code - 96
    } else {
        code + 27 - 65
    }
}

fn priority_sum(str: String) -> i32 {
    let mut result = 0;

    for line in str.split("\n") {
        let entry = line.trim();
        if entry.len() == 0 {
            continue;
        }

        let repeat = find_repeat(entry);
        result += priority(repeat);
    }

    result
}

fn priority_sum3(str: String) -> i32 {
    let mut result = 0;
    let lines: Vec<&str> = str
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect();

    for chunk in lines.chunks(3) {
        let repeat = find_repeat3(chunk[0], chunk[1], chunk[2]);
        result += priority(repeat);
    }

    result
}

#[test]
fn example1() {
    let result = priority_sum(String::from(
        "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    ));

    assert_eq!(result, 157);
}

#[test]
fn example2() {
    let result = priority_sum3(String::from(
        "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    ));

    assert_eq!(result, 70);
}

fn main() {
    let input = fs::read_to_string("../input/day-3.txt").expect("Unable to read file");
    // println!("{}", priority_sum(input));
    println!("{}", priority_sum3(input));
}
