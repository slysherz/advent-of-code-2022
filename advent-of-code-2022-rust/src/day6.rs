use std::fs;

fn solve(str: String, size: usize) -> usize {
    'outer: for (i, win) in str.as_bytes().windows(size).enumerate() {
        for (j, c1) in win.iter().enumerate() {
            for c2 in &win[j + 1..] {
                if *c2 == *c1 {
                    continue 'outer;
                }
            }
        }

        return i + size;
    }

    std::unreachable!()
}

fn solve1(str: String) -> usize {
    solve(str, 4)
}

fn solve2(str: String) -> usize {
    solve(str, 14)
}

#[test]
fn example1() {
    assert_eq!(solve1(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
    assert_eq!(solve1(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
    assert_eq!(solve1(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
    assert_eq!(
        solve1(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
        10
    );
    assert_eq!(solve1(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
}

#[test]
fn example2() {
    assert_eq!(solve2(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
    assert_eq!(solve2(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
    assert_eq!(solve2(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 23);
    assert_eq!(
        solve2(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
        29
    );
    assert_eq!(solve2(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
}

fn main() {
    let input = fs::read_to_string("../input/day-6.txt").expect("Unable to read file");
    // println!("{}", solve1(input));
    println!("{}", solve2(input));
}
