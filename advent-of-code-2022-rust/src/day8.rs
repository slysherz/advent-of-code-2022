use std::fs;

fn pos(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn parse_trees(str: String) -> (Vec<u8>, usize, usize) {
    let mut trees: Vec<u8> = Vec::new();
    let mut counted: Vec<bool> = Vec::new();
    let mut width = 0;
    for line in str.split('\n') {
        if line == "" {
            continue;
        }

        width = line.len();
        for c in line.chars() {
            trees.push(String::from(c).parse::<u8>().unwrap());
            counted.push(false);
        }
    }
    let height = trees.len() / width;

    (trees, width, height)
}

fn solve1(str: String) -> usize {
    let (trees, width, height) = parse_trees(str);
    let mut counted: Vec<bool> = vec![false; width * height];
    for y in 0..height {
        {
            let mut min: i32 = -1;
            for x in 0..width {
                let p = pos(x, y, width);
                if i32::from(trees[p]) > min {
                    counted[p] = true;
                    min = i32::from(trees[p]);
                }
            }
        }

        {
            let mut min: i32 = -1;
            for x in (0..width).rev() {
                let p = pos(x, y, width);
                if i32::from(trees[p]) > min {
                    counted[p] = true;
                    min = i32::from(trees[p]);
                }
            }
        }
    }

    for x in 0..width {
        {
            let mut min: i32 = -1;
            for y in 0..height {
                let p = pos(x, y, width);
                if i32::from(trees[p]) > min {
                    counted[p] = true;
                    min = i32::from(trees[p]);
                }
            }
        }
        {
            let mut min: i32 = -1;
            for y in (0..height).rev() {
                let p = pos(x, y, width);
                if i32::from(trees[p]) > min {
                    counted[p] = true;
                    min = i32::from(trees[p]);
                }
            }
        }
    }

    counted.iter().filter(|t| **t).count()
}

fn scenic_score_range<I1, I2>(trees: &Vec<u8>, width: usize, min: u8, rx: I1, ry: I2) -> usize
where
    I1: Iterator<Item = usize>,
    I2: Iterator<Item = usize>,
{
    let mut score = 0;
    let temp: Vec<usize> = ry.collect();

    for x in rx {
        for y in &temp {
            score += 1;
            if trees[pos(x, *y, width)] >= min {
                return score;
            }
        }
    }

    score
}

fn solve2(str: String) -> usize {
    let (trees, width, height) = parse_trees(str);

    let mut max_scenic_score = 0;
    for x1 in 0..width {
        for y1 in 0..height {
            let min = trees[pos(x1, y1, height)];
            let score = 1
                * scenic_score_range(&trees, width, min, x1..x1 + 1, (0..y1).rev()) // UP
                * scenic_score_range(&trees, width, min, (0..x1).rev(), y1..y1 + 1) // LEFT
                * scenic_score_range(&trees, width, min, x1..x1 + 1, y1 + 1..height)// DOWN
                * scenic_score_range(&trees, width, min, x1 + 1..width, y1..y1 + 1) // RIGHT
                ;

            max_scenic_score = usize::max(max_scenic_score, score);
        }
    }

    max_scenic_score
}

#[test]
fn example1() {
    let result = solve1(String::from(
        "
30373
25512
65332
33549
35390",
    ));

    assert_eq!(result, 21);
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "
30373
25512
65332
33549
35390",
    ));

    assert_eq!(result, 8);
}

fn main() {
    let input = fs::read_to_string("../input/day-8.txt").expect("Unable to read file");
    // println!("{}", solve1(input));
    println!("{}", solve2(input));
}
