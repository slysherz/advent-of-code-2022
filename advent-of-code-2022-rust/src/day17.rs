use std::{collections::HashMap, fs};

type Rock = Vec<u16>;
type PackedRock = u64;

fn parse(str: &str) -> Vec<bool> {
    str.chars().map(|c| c == '<').collect()
}

fn left(shape_line: PackedRock, units: usize) -> PackedRock {
    shape_line >> units
}

fn right(shape_line: PackedRock, units: usize) -> PackedRock {
    shape_line << units
}

fn pack(a1: u16, a2: u16, a3: u16, a4: u16) -> PackedRock {
    ((a1 as PackedRock) << 0)
        + ((a2 as PackedRock) << 16)
        + ((a3 as PackedRock) << 32)
        + ((a4 as PackedRock) << 48)
}

fn pack_rock(rock: Rock) -> PackedRock {
    assert!(rock.len() <= 4);
    pack(
        *rock.get(0).unwrap_or(&0),
        *rock.get(1).unwrap_or(&0),
        *rock.get(2).unwrap_or(&0),
        *rock.get(3).unwrap_or(&0),
    )
}

fn hits_packed(ground: PackedRock, shape: PackedRock) -> bool {
    ground & (shape ^ ground) != ground
}

fn hits(ground: &Rock, &shape: &PackedRock, ofset_y: usize) -> bool {
    let g = pack(
        *ground.get(0 + ofset_y).unwrap_or(&0),
        *ground.get(1 + ofset_y).unwrap_or(&0),
        *ground.get(2 + ofset_y).unwrap_or(&0),
        *ground.get(3 + ofset_y).unwrap_or(&0),
    );

    return hits_packed(g, shape);
}

fn free_rows(ground: &Rock) -> usize {
    for (i, &row) in ground.iter().enumerate().rev() {
        if row != 0 {
            return ground.len() - i - 1;
        }
    }

    0
}

fn draw_rock(rock: &Rock, name: &str) {
    let mut result = String::from(name) + "\n";
    for line in rock.iter().rev() {
        for i in 0..9 {
            let dot = if i == 0 || i == 8 {
                "|"
            } else if line & 1 << i != 0 {
                "#"
            } else {
                "."
            };
            result += dot;
        }
        result += "\n"
    }
    println!("{}", result);
}

fn simulate_rock(ground: &mut Rock, jets: &Vec<bool>, tick: &mut usize, mut rock: PackedRock) {
    let edge = 0b100000001;
    let walls: PackedRock = pack_rock(vec![edge, edge, edge, edge]);
    rock = right(rock, 3);
    let free = free_rows(&ground);
    for _ in 0..(4 - free) {
        ground.push(0);
    }
    let height = ground.len() - 1;

    for h in (0..height).rev() {
        // Apply jet
        let new_rock = if jets[*tick % jets.len()] {
            left(rock, 1)
        } else {
            right(rock, 1)
        };
        *tick += 1;

        rock = if hits_packed(walls, new_rock) || hits(&ground, &new_rock, h + 1) {
            rock
        } else {
            new_rock
        };

        // Try to land the rock
        if hits(&ground, &rock, h) {
            ground[1 + h] |= (rock >> 0 & 0b111111111) as u16;
            ground[2 + h] |= (rock >> 16 & 0b111111111) as u16;
            ground[3 + h] |= (rock >> 32 & 0b111111111) as u16;
            ground[4 + h] |= (rock >> 48 & 0b111111111) as u16;

            // draw_rock(&ground, "GROUND");
            break;
        }
    }
}

fn tower_height(ground: &Rock) -> usize {
    ground.len() - free_rows(&ground)
}

// Quick way to compare if the top of the tower looks the same
fn heuristic(ground: &Rock) -> PackedRock {
    let height = ground.len() - free_rows(ground);
    pack_rock(ground[height - 4..height].to_vec())
}

fn find_rock_loop(
    mut ground: Rock,
    jets: &Vec<bool>,
    shapes: &Vec<PackedRock>,
    steps: usize,
) -> Option<((usize, usize, usize), (usize, usize, usize))> {
    let warmup = 5;

    let mut tick = 0;
    let mut shape_id = 0;
    for _ in 0..warmup {
        simulate_rock(
            &mut ground,
            &jets,
            &mut tick,
            shapes[shape_id % shapes.len()],
        );
        shape_id += 1;
        tick = tick % jets.len();
    }

    let mut log: HashMap<(usize, usize, u64), (usize, usize, usize)> = HashMap::new();
    for _ in warmup..steps {
        let key1 = (
            shape_id % shapes.len(),
            tick % jets.len(),
            heuristic(&ground),
        );
        let current_height = tower_height(&ground);
        let new_entry = (current_height, shape_id, tick);

        if let Some(&entry) = log.get(&key1) {
            return Some((entry, new_entry));
        }

        simulate_rock(
            &mut ground,
            &jets,
            &mut tick,
            shapes[shape_id % shapes.len()],
        );
        log.insert(key1, new_entry);
        shape_id += 1;
    }

    None
}

fn solve(str: &str, steps: usize) -> usize {
    let jets = parse(str);
    let start_ground: Rock = vec![0b111111111];
    let shapes: Vec<PackedRock> = vec![
        vec![0b1111],
        vec![0b010, 0b111, 0b010],
        vec![0b111, 0b100, 0b100],
        vec![0b1, 0b1, 0b1, 0b1],
        vec![0b11, 0b11],
    ]
    .iter()
    .map(|r| pack_rock(r.to_vec()))
    .collect();

    // Find out when the tower starts repeating
    let res = find_rock_loop(start_ground.clone(), &jets, &shapes, steps);
    if res.is_none() {
        let mut ground = start_ground.clone();
        let mut tick: usize = 0;
        for i in 0..steps {
            simulate_rock(&mut ground, &jets, &mut tick, shapes[i % shapes.len()]);
        }
        return tower_height(&ground) - 1;
    }

    let ((h1, s1, _t1), (h2, s2, _t2)) = res.unwrap();
    let h_diff = h2 - h1;
    let s_diff = s2 - s1;
    let shapes_left = (steps - s1) % s_diff;
    let repeats = (steps - s1) / s_diff;

    let mut tick: usize = 0;
    let mut ground = start_ground;

    for i in 0..(s1 + shapes_left) {
        simulate_rock(&mut ground, &jets, &mut tick, shapes[i % shapes.len()]);
    }

    tower_height(&ground) + h_diff * repeats - 1
}

#[test]
fn example1() {
    let result = solve(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022);
    assert_eq!(result, 3068);
}

#[test]
fn example2() {
    let result = solve(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 1000000000000);
    assert_eq!(result, 1514285714288);
}

fn main() {
    let input = fs::read_to_string("../input/day-17.txt").expect("Unable to read file");
    // println!("\n{}", solve(input.as_str(), 2022));
    println!("\n{}", solve(input.as_str(), 1000000000000));
}
