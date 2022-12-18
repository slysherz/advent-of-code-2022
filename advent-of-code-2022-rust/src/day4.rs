use std::fs;

#[derive(Clone, Copy)]
struct Range {
    min: i32,
    max: i32,
}

fn parse_range(str: &str) -> Range {
    let mut it = str.split('-');
    let min = it.next().unwrap();
    let max = it.next().unwrap();

    Range {
        min: min.parse::<i32>().unwrap(),
        max: max.parse::<i32>().unwrap(),
    }
}

fn contained_in(r1: Range, r2: Range) -> bool {
    r1.min >= r2.min && r1.max <= r2.max
}

fn overlaps_with(r1: Range, r2: Range) -> bool {
    let is_less = r2.max < r1.min;
    let is_more = r2.min > r1.max;
    !is_less && !is_more
}

fn count_contained(str: String) -> i32 {
    let mut result = 0;

    for line in str.split('\n') {
        let entry = line.trim();
        if entry.len() == 0 {
            continue;
        }

        let mut it = entry.split(',');
        let range1 = parse_range(it.next().unwrap());
        let range2 = parse_range(it.next().unwrap());

        if contained_in(range1, range2) || contained_in(range2, range1) {
            result += 1;
        }
    }

    result
}

fn count_overlaps(str: String) -> i32 {
    let mut result = 0;

    for line in str.split('\n') {
        let entry = line.trim();
        if entry.len() == 0 {
            continue;
        }

        let mut it = entry.split(',');
        let range1 = parse_range(it.next().unwrap());
        let range2 = parse_range(it.next().unwrap());

        if overlaps_with(range1, range2) {
            result += 1;
        }
    }

    result
}

#[test]
fn example1() {
    let result = count_contained(String::from(
        "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    ));

    assert_eq!(result, 2);
}

#[test]
fn example2() {
    let result = count_overlaps(String::from(
        "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    ));

    assert_eq!(result, 4);
}

fn main() {
    let input = fs::read_to_string("../input/day-4.txt").expect("Unable to read file");
    // println!("{}", count_contained(input));
    println!("{}", count_overlaps(input));
}
