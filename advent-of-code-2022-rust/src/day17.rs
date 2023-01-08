use std::fs;

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

fn solve(str: &str, steps: usize) -> usize {
    let destroy_cutoff = 10000;
    let max_fall_distance = 200;
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

    let edge = 0b100000001;
    let walls: PackedRock = pack_rock(vec![edge, edge, edge, edge]);
    let jets = parse(str);
    let mut ground: Rock = vec![0b111111111];
    let mut tick = 0;
    let mut max_descent = 0;
    let mut destroyed_rows = 0;

    for rock_id in 0..steps {
        if ground.len() > destroy_cutoff + max_fall_distance {
            ground = ground[destroy_cutoff..ground.len()].to_vec();
            destroyed_rows += destroy_cutoff;
        }

        if rock_id % (1000000000000 / 10000) == 0 {
            println!(
                "{} | 10000  {}",
                rock_id / (1000000000000 / 10000),
                max_descent
            );
        }

        let mut rock = right(shapes[rock_id % shapes.len()], 3);
        let free = free_rows(&ground);
        for _ in 0..(4 - free) {
            ground.push(0);
        }
        let height = ground.len() - 1;
        // draw_rock(&rock, "ROCK");

        for h in (0..height).rev() {
            // Apply jet
            let new_rock = if jets[tick % jets.len()] {
                left(rock, 1)
            } else {
                right(rock, 1)
            };
            tick += 1;

            rock = if hits_packed(walls, new_rock) || hits(&ground, &new_rock, h + 1) {
                rock
            } else {
                new_rock
            };
            // draw_rock(&rock, "ROCK");

            // Try to land the rock
            if hits(&ground, &rock, h) {
                ground[1 + h] |= (rock >> 0 & 0b111111111) as u16;
                ground[2 + h] |= (rock >> 16 & 0b111111111) as u16;
                ground[3 + h] |= (rock >> 32 & 0b111111111) as u16;
                ground[4 + h] |= (rock >> 48 & 0b111111111) as u16;

                // draw_rock(&ground, "GROUND");
                max_descent = usize::max(max_descent, ground.len() - h);
                break;
            }
        }
    }

    destroyed_rows + ground.len() - free_rows(&ground) - 1
}

#[test]
fn example1() {
    let result = solve(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022);
    assert_eq!(result, 3068);
}

fn example2() {
    let result = solve(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 1000000000000);
    assert_eq!(result, 3068);
}

fn main() {
    let input = fs::read_to_string("../input/day-17.txt").expect("Unable to read file");
    // println!("\n{}", solve(input.as_str(), 2022));
    println!("\n{}", solve(input.as_str(), 1000000000000));
}
