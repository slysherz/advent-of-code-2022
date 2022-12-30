use std::{cmp::Ordering, collections::HashSet, fs};

type Pos = (i32, i32);
type Line = (Pos, Pos);
type Rocks = Vec<Line>;
type Sand = HashSet<Pos>;

fn intersects_line(((x1, y1), (x2, y2)): Line, (x, y): Pos) -> bool {
    (x1 <= x && x <= x2 && y1 == y2 && y1 == y) || (x1 == x2 && x == x1 && y1 <= y && y <= y2)
}

fn intersects_rock(rocks: &Rocks, unit: Pos) -> bool {
    rocks.iter().any(|line| intersects_line(*line, unit))
}

fn parse_rocks(str: &str) -> Rocks {
    let mut result = Vec::new();
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        let mut last = (0, 0);
        for point_str in line.split(" -> ") {
            if let [xstr, ystr] = point_str.split(',').collect::<Vec<&str>>().as_slice() {
                let pos: Pos = (xstr.parse().unwrap(), ystr.parse().unwrap());
                if last != (0, 0) {
                    if (pos.0 == last.0 && pos.1 < last.1) || pos.0 < last.0 {
                        result.push((pos, last));
                    } else {
                        result.push((last, pos));
                    }
                }
                last = pos;
            }
        }
    }

    result
}

fn solve1(str: &str) -> usize {
    let rocks = parse_rocks(str);
    let max_y = rocks.iter().fold(0, |r, ((_, y), _)| i32::max(r, *y));
    let mut sand: Sand = HashSet::new();

    'unit: while true {
        // for every unit of sand
        let mut unit = (500, 0);
        'tick: while unit.1 < max_y {
            // for every tick
            let possibilities: [Pos; 3] = [(0, 1), (-1, 1), (1, 1)];
            for (dx, dy) in possibilities {
                let next = (unit.0 + dx, unit.1 + dy);
                if !sand.contains(&next) && !intersects_rock(&rocks, next) {
                    unit = next;
                    continue 'tick;
                }
            }

            sand.insert(unit);
            continue 'unit;
        }

        break;
    }

    sand.len()
}

fn solve2(str: &str) -> usize {
    let mut rocks = parse_rocks(str);
    let max_y = rocks.iter().fold(0, |r, ((_, y), _)| i32::max(r, *y));
    rocks.push(((0, max_y + 2), (1000, max_y + 2))); // kinda hacky, but should be enough
    let mut sand: Sand = HashSet::new();

    'unit: while true {
        // for every unit of sand
        let mut unit = (500, 0);
        'tick: while unit.1 < max_y + 2 {
            // for every tick
            let possibilities: [Pos; 3] = [(0, 1), (-1, 1), (1, 1)];
            for (dx, dy) in possibilities {
                let next = (unit.0 + dx, unit.1 + dy);
                if !sand.contains(&next) && !intersects_rock(&rocks, next) {
                    unit = next;
                    continue 'tick;
                }
            }

            sand.insert(unit);
            if unit == (500, 0) {
                break 'unit;
            }
            continue 'unit;
        }

        std::unreachable!()
    }

    sand.len()
}

#[test]
fn example1() {
    let result = solve1(
        "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    assert_eq!(result, 24);
}

#[test]
fn example2() {
    let result = solve2(
        "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    assert_eq!(result, 93);
}

fn main() {
    let input = fs::read_to_string("../input/day-14.txt").expect("Unable to read file");
    // println!("\n{}", solve1(input.as_str()));
    println!("\n{}", solve2(input.as_str()));
}
