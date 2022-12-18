use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy)]
enum Score {
    Win,
    Loss,
    Draw,
}

fn parse_play(play: char) -> RPS {
    match play {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => std::unreachable!(),
    }
}

fn parse_result(play: char) -> Score {
    match play {
        'X' => Score::Loss,
        'Y' => Score::Draw,
        'Z' => Score::Win,
        _ => std::unreachable!(),
    }
}

fn desired_result_score(opponent: RPS, desired_score: Score) -> i32 {
    let tries = vec![RPS::Rock, RPS::Paper, RPS::Scissors];
    for you in tries {
        if play_result(opponent, you) == desired_score {
            return play_score(opponent, you);
        }
    }

    std::unreachable!()
}

fn play_result(opponent: RPS, you: RPS) -> Score {
    match (opponent, you) {
        (RPS::Rock, RPS::Paper) => Score::Win,
        (RPS::Paper, RPS::Scissors) => Score::Win,
        (RPS::Scissors, RPS::Rock) => Score::Win,
        (RPS::Rock, RPS::Scissors) => Score::Loss,
        (RPS::Paper, RPS::Rock) => Score::Loss,
        (RPS::Scissors, RPS::Paper) => Score::Loss,
        _ => Score::Draw,
    }
}

fn play_score(opponent: RPS, you: RPS) -> i32 {
    let a = match play_result(opponent, you) {
        Score::Win => 6,
        Score::Draw => 3,
        Score::Loss => 0,
    };

    let b = match you {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    a + b
}

fn calculate_score1(str: String) -> i32 {
    let mut result = 0;

    for line in str.split("\n") {
        let play = line.trim();
        if play.len() < 3 {
            continue;
        }

        let opponent = parse_play(play.chars().nth(0).unwrap());
        let you = parse_play(play.chars().nth(2).unwrap());
        result += play_score(opponent, you)
    }

    result
}

fn calculate_score2(str: String) -> i32 {
    let mut result = 0;

    for line in str.split("\n") {
        let play = line.trim();
        if play.len() < 3 {
            continue;
        }

        let opponent = parse_play(play.chars().nth(0).unwrap());
        let you = parse_result(play.chars().nth(2).unwrap());
        result += desired_result_score(opponent, you)
    }

    result
}

#[test]
fn day2_example1() {
    let score = calculate_score1(String::from(
        "A Y
        B X
        C Z
    ",
    ));

    assert_eq!(score, 15);
}

#[test]
fn day2_example2() {
    let score = calculate_score2(String::from(
        "A Y
        B X
        C Z
    ",
    ));

    assert_eq!(score, 12);
}

fn main() {
    let input = fs::read_to_string("../input/day-2.txt").expect("Unable to read file");
    // println!("{}", calculate_score1(input));
    println!("{}", calculate_score2(input));
}
