use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    None,
    Up,
    Down,
    Left,
    Right,
}

fn parse_map(str: String) -> (Vec<u8>, usize, usize, usize, usize) {
    let mut map: Vec<u8> = Vec::new();
    let mut width = 0;
    let mut start_pos = 0;
    let mut end_pos = 0;
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        width = line.len();
        for c in line.chars() {
            if c == 'S' {
                start_pos = map.len();
                map.push(0);
            } else if c == 'E' {
                end_pos = map.len();
                map.push('z' as u8 - 'a' as u8);
            } else {
                map.push(c as u8 - 'a' as u8);
            }
        }
    }

    let height = map.len() / width;
    (map, width, height, start_pos, end_pos)
}

fn pos(index: usize, width: usize) -> (usize, usize) {
    (index / width, index % width)
}

fn ind(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn rev(dir: Dir) -> Dir {
    match dir {
        Dir::Down => Dir::Up,
        Dir::Up => Dir::Down,
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
        _ => std::unreachable!(),
    }
}

fn neighbour(index: usize, width: usize, height: usize, dir: Dir) -> Option<usize> {
    match (index % width, index / width, dir) {
        (x, y, Dir::Left) => {
            if x <= 0 {
                None
            } else {
                Some(ind(x - 1, y, width))
            }
        }
        (x, y, Dir::Right) => {
            if x + 1 >= width {
                None
            } else {
                Some(ind(x + 1, y, width))
            }
        }
        (x, y, Dir::Down) => {
            if y <= 0 {
                None
            } else {
                Some(ind(x, y - 1, width))
            }
        }
        (x, y, Dir::Up) => {
            if y + 1 >= height {
                None
            } else {
                Some(ind(x, y + 1, width))
            }
        }
        _ => std::unreachable!(),
    }
}

fn shortest_path(map: &Vec<u8>, width: usize, height: usize, end_pos: usize) -> Vec<Dir> {
    let mut path: Vec<Dir> = map.iter().map(|_| Dir::None).collect();
    let mut next = VecDeque::new();
    next.push_back(end_pos);

    while next.len() > 0 {
        let index = next.pop_front().unwrap();

        let val = map[index];
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            if let Some(i) = neighbour(index, width, height, dir) {
                if map[i] + 1 < val || path[i] != Dir::None {
                    continue;
                }
                next.push_back(i);
                path[i] = rev(dir);
            }
        }
    }

    path
}

fn solve1(str: String) -> usize {
    let (map, width, height, start_pos, end_pos) = parse_map(str);
    let path = shortest_path(&map, width, height, end_pos);

    let mut pos = start_pos;
    let mut count = 0;
    while pos != end_pos {
        pos = neighbour(pos, width, height, path[pos]).unwrap();
        count += 1
    }

    count
}

fn solve2(str: String) -> usize {
    let (map, width, height, start_pos, end_pos) = parse_map(str);
    let path = shortest_path(&map, width, height, end_pos);

    let mut min_count = usize::MAX;
    for i in 0..map.len() {
        if map[i] != 0 || path[i] == Dir::None {
            continue;
        }

        let mut pos = i;
        let mut count = 0;
        while pos != end_pos {
            pos = neighbour(pos, width, height, path[pos]).unwrap();
            count += 1
        }
        min_count = usize::min(min_count, count);
    }

    min_count
}

#[test]
fn example1() {
    let result = solve1(String::from(
        "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    ));

    assert_eq!(result, 31)
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    ));

    assert_eq!(result, 29)
}

fn main() {
    let input = fs::read_to_string("../input/day-12.txt").expect("Unable to read file");
    // println!("\n{}", solve1(input));
    println!("\n{}", solve2(input));
}
