use std::fs;

fn get_most_calories(str: String) -> i32 {
    let mut group_max = 0;
    let mut current_group = 0;

    for line in str.split("\n") {
        let res = line.trim().parse::<i32>();
        match res {
            Ok(val) => current_group += val,
            _ => {
                group_max = std::cmp::max(group_max, current_group);
                current_group = 0;
            }
        }
    }

    std::cmp::max(group_max, current_group)
}

#[test]
fn day1_example1() {
    let result = get_most_calories(String::from(
        "
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000",
    ));

    assert_eq!(result, 24000);
}

fn get_top3_most_calories(str: String) -> i32 {
    let mut groups = Vec::new();
    let mut current_group = 0;

    for line in str.split("\n") {
        let res = line.trim().parse::<i32>();
        match res {
            Ok(val) => current_group += val,
            _ => {
                groups.push(current_group);
                current_group = 0;
            }
        }
    }

    groups.push(current_group);
    groups.sort_by(|a, b| b.cmp(a));
    groups[0..3].iter().sum()
}

#[test]
fn day1_example2() {
    let result = get_top3_most_calories(String::from(
        "
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000",
    ));

    assert_eq!(result, 45000);
}

fn main() {
    let input = fs::read_to_string("../input/day-1.txt").expect("Unable to read file");
    // println!("{}", get_most_calories(input));
    println!("{}", get_top3_most_calories(input));
}
