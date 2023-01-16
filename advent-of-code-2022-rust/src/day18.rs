use std::{
    collections::{HashSet, VecDeque},
    fs,
};

type Point = (i32, i32, i32);

fn parse(str: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        let temp = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        if let [x, y, z] = temp[0..3] {
            result.insert((x, y, z));
        }
    }

    result
}

fn solve1(str: &str) -> usize {
    let points = parse(str);
    let mut repeats = 0;
    for &key in &points {
        let (x, y, z) = key;
        for neighbor in [
            (x, y, z - 1),
            (x, y, z + 1),
            (x, y - 1, z),
            (x, y + 1, z),
            (x - 1, y, z),
            (x + 1, y, z),
        ] {
            if points.contains(&neighbor) {
                repeats += 1;
            }
        }
    }

    return points.len() * 6 - repeats;
}

fn point_max((x1, y1, z1): Point, (x2, y2, z2): Point) -> Point {
    (i32::max(x1, x2), i32::max(y1, y2), i32::max(z1, z2))
}

fn point_min((x1, y1, z1): Point, (x2, y2, z2): Point) -> Point {
    (i32::min(x1, x2), i32::min(y1, y2), i32::min(z1, z2))
}

fn dfs(
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
    points: &HashSet<Point>,
) -> HashSet<Point> {
    let mut reacheable = HashSet::new();

    let mut stack = VecDeque::new();
    stack.push_back((min_x, min_y, min_z));

    while stack.len() > 0 {
        let (x, y, z) = stack.pop_back().unwrap();

        if reacheable.contains(&(x, y, z)) {
            continue;
        }

        reacheable.insert((x, y, z));
        for (x, y, z) in [
            (x, y, z - 1),
            (x, y, z + 1),
            (x, y - 1, z),
            (x, y + 1, z),
            (x - 1, y, z),
            (x + 1, y, z),
        ] {
            if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
                continue;
            }

            if points.contains(&(x, y, z)) {
                continue;
            }
            stack.push_back((x, y, z));
        }
    }

    reacheable
}

fn solve2(str: &str) -> usize {
    let points = parse(str);
    let (min_x, min_y, min_z) = points.clone().into_iter().reduce(point_min).unwrap();
    let (max_x, max_y, max_z) = points.clone().into_iter().reduce(point_max).unwrap();

    let reachable = dfs(
        min_x - 1,
        max_x + 1,
        min_y - 1,
        max_y + 1,
        min_z - 1,
        max_z + 1,
        &points,
    );
    let mut result = 0;
    for &key in &points {
        let (x, y, z) = key;
        for neighbor in [
            (x, y, z - 1),
            (x, y, z + 1),
            (x, y - 1, z),
            (x, y + 1, z),
            (x - 1, y, z),
            (x + 1, y, z),
        ] {
            if reachable.contains(&neighbor) {
                result += 1;
            }
        }
    }

    result
}

#[test]
fn example1() {
    let result = solve1(
        "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );
    assert_eq!(result, 64);
}

#[test]
fn example2() {
    let result = solve2(
        "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );
    assert_eq!(result, 58);
}

fn main() {
    let input = fs::read_to_string("../input/day-18.txt").expect("Unable to read file");
    // println!("\n{}", solve1(input.as_str()));
    println!("\n{}", solve2(input.as_str()));
}
