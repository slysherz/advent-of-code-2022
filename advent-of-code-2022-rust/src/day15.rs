use std::{collections::HashSet, fs};

type Pos = (i32, i32);
type Sensor = (Pos, Pos);
type Row = Vec<Pos>;

fn parse_pos(str: &str) -> Pos {
    if let [x, y] = str
        .split(", y=")
        .into_iter()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        return (x.parse().unwrap(), y.parse().unwrap());
    }

    std::unreachable!()
}

fn parse(str: &str) -> Vec<Sensor> {
    let mut result = Vec::new();

    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        if let [s, b] = line
            .split(": closest beacon is at x=")
            .into_iter()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            result.push((parse_pos(&s[12..]), parse_pos(b)))
        } else {
            std::unreachable!()
        }
    }

    result
}

fn dist((x1, y1): Pos, (x2, y2): Pos) -> i32 {
    i32::abs(x1 - x2) + i32::abs(y1 - y2)
}

fn overlaps((x1, x2): Pos, (x3, x4): Pos) -> bool {
    !(x2 < x3 || x1 > x4)
}

fn merge((x1, x2): Pos, (x3, x4): Pos) -> Pos {
    (i32::min(x1, x3), i32::max(x2, x4))
}

fn insert_range(ranges: &mut Vec<Pos>, range: Pos) {
    if range.1 - range.0 < 0 {
        return;
    }
    let xrange = ranges.iter().fold(range, |r1, r2| {
        if overlaps(r1, *r2) {
            merge(r1, *r2)
        } else {
            r1
        }
    });

    ranges.retain(|r| !overlaps(*r, xrange));
    ranges.push(xrange);
}

fn solve1(str: &str, line: i32) -> i32 {
    let sensors = parse(str);
    let mut ranges: Vec<Pos> = Vec::new();
    let mut beacons: HashSet<Pos> = HashSet::new();

    for (sensor, beacon) in sensors {
        let d = dist(sensor, beacon);
        let y_dist = i32::abs(sensor.1 - line);
        let range_size = d - y_dist;
        insert_range(&mut ranges, (sensor.0 - range_size, sensor.0 + range_size));

        if line == beacon.1 {
            beacons.insert(beacon);
        }
    }

    ranges.iter().fold(0, |res, (x1, x2)| res + (x2 - x1) + 1) - (beacons.len() as i32)
}

fn solve2(str: &str, size: i32) -> i64 {
    let sensors = parse(str);

    for y in 0..size {
        let mut ranges: Vec<Pos> = Vec::new();
        let mut beacons: HashSet<Pos> = HashSet::new();

        for (sensor, beacon) in &sensors {
            let d = dist(*sensor, *beacon);
            let y_dist = i32::abs(sensor.1 - y);
            let range_size = d - y_dist;
            insert_range(&mut ranges, (sensor.0 - range_size, sensor.0 + range_size));

            if y == beacon.1 {
                beacons.insert(*beacon);
            }
        }

        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        let mut last_x = -1;
        for (x1, x2) in ranges {
            for x in (last_x + 1)..x1 {
                if !beacons.contains(&(x, y)) {
                    return (x as i64) * 4000000 + y as i64;
                }
            }
            last_x = x2;
        }
    }

    std::unreachable!()
}

#[test]
fn example1() {
    let result = solve1(
        "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        10,
    );
    assert_eq!(result, 26);
}

#[test]
fn example2() {
    let result = solve2(
        "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        20,
    );
    assert_eq!(result, 56000011);
}

fn main() {
    let input = fs::read_to_string("../input/day-15.txt").expect("Unable to read file");
    // println!("\n{}", solve1(input.as_str(), 2000000));
    println!("\n{}", solve2(input.as_str(), 4000000));
}
