use std::fs;

struct Dir {
    parent: usize,
    size: usize,
    name: String, // Not needed because names never repeat
}

fn parse_directory(str: String) -> Vec<Dir> {
    let mut dirs: Vec<Dir> = Vec::new();
    dirs.push(Dir {
        parent: 0,
        size: 0,
        name: String::from("/"),
    });

    let mut current_dir = 0;
    for line in str.split('\n') {
        if line == "" || line.starts_with("$ cd /") {
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir ") {
        } else if line.starts_with("$ cd ..") {
            current_dir = dirs[current_dir].parent;
        } else if line.starts_with("$ cd ") {
            dirs.push(Dir {
                parent: current_dir,
                size: 0,
                name: String::from(&line[6..]),
            });
            current_dir = dirs.len() - 1;
        } else {
            let mut iter = line.trim().split(" ");
            let size = iter.next().unwrap().parse::<usize>().unwrap();
            // let name = String::from(iter.next().unwrap());

            let mut parent = current_dir;
            while parent != 0 {
                dirs[parent].size += size;
                parent = dirs[parent].parent;
            }

            dirs[0].size += size;
        }
    }

    dirs
}

fn solve1(str: String) -> usize {
    let dirs = parse_directory(str);

    let mut sum = 0;
    for dir in dirs {
        if dir.size <= 100000 {
            sum += dir.size;
        }
    }

    sum
}

fn solve2(str: String) -> usize {
    let dirs = parse_directory(str);

    let total_disk = 70000000;
    let needed_for_update = 30000000;
    let used_disk = dirs[0].size;
    let needs_to_free = used_disk + needed_for_update - total_disk;

    let mut min = usize::MAX;
    for dir in dirs {
        if dir.size >= needs_to_free {
            min = usize::min(min, dir.size);
        }
    }

    min
}

#[test]
fn example1() {
    let result = solve1(String::from(
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    ));

    assert_eq!(result, 95437);
}

#[test]
fn example2() {
    let result = solve2(String::from(
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    ));

    assert_eq!(result, 24933642);
}

fn main() {
    let input = fs::read_to_string("../input/day-7.txt").expect("Unable to read file");
    // println!("{}", solve1(input));
    println!("{}", solve2(input));
}
