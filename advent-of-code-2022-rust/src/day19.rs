use std::fs;

type Types = [i32; 4];
type Status = (Types, Types);
type Recipe = (Types, usize);

struct Blueprint {
    id: usize,
    ore_cost: i32,
    clay_cost: i32,
    obsidian_ore_cost: i32,
    obsidian_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obsidian_cost: i32,
}

fn map_types<F: Fn((i32, usize)) -> i32>(arr: Types, f: F) -> Types {
    let mut res = [0; 4];
    for i in 0..res.len() {
        res[i] = f((arr[i], i));
    }

    res
}

fn buy((resources, mut robots): Status, recipe: Types, amount: i32, kind: usize) -> (Types, Types) {
    let res = map_types(resources, |(v, i)| v - recipe[i] * amount);
    robots[kind] += amount;
    (res, robots)
}

fn is_valid(arr: Types) -> bool {
    arr.iter().all(|&n| n >= 0)
}

// All resources in A are less than B
fn all_less_or_same([a1, a2, a3, a4]: Types, [b1, b2, b3, b4]: Types) -> bool {
    a1 <= b1 && a2 <= b2 && a3 <= b3 && a4 <= b4
}

fn buy_possibilities(state: Status, recipes: &[Recipe], out: &mut Vec<Status>) {
    if recipes.len() == 0 {
        for entry in out.iter() {
            if all_less_or_same(state.0, entry.0) && all_less_or_same(state.1, entry.1) {
                return;
            }
        }
        out.push(state);
        return;
    }

    for i in 0..i32::MAX {
        let (recipe, kind) = recipes[0];
        let next = buy(state, recipe, i, kind);
        if !is_valid(next.0) {
            break;
        }

        buy_possibilities(next, &recipes[1..], out);
    }
}

fn tick(&(resources, robots): &Status) -> Status {
    (map_types(resources, |(v, i)| v + robots[i]), robots)
}

fn bp_recipes(bp: Blueprint) -> Vec<Recipe> {
    vec![
        ([bp.ore_cost, 0, 0, 0], 0),
        ([bp.clay_cost, 0, 0, 0], 1),
        ([bp.obsidian_ore_cost, bp.obsidian_clay_cost, 0, 0], 2),
        ([bp.geode_ore_cost, 0, bp.geode_obsidian_cost, 0], 3),
    ]
}

fn optimize(recipes: Vec<Recipe>, id: usize) -> i32 {
    let minutes = 24;
    let mut result = vec![([0, 0, 0, 0], [1, 0, 0, 0])];
    for minute in 0..minutes {
        println!("[{id}] MINUTE {minute} POSSIBILITIES = {}", result.len());
        let mut next = Vec::new();
        for entry in result {
            buy_possibilities(entry, &recipes, &mut next);
        }

        result = next.iter().map(tick).collect();
    }

    let mut max = 0;
    for (resources, _) in result {
        max = i32::max(max, resources[3]);
    }

    max
}

fn solve1(input: Vec<Blueprint>) -> i32 {
    let mut result = 0;
    for bp in input {
        let id = bp.id;
        let recipes = bp_recipes(bp);
        let res = optimize(recipes, id);
        result += res * id as i32;
    }

    result
}

#[test]
fn example1() {
    let result = solve1(vec![
        Blueprint {
            id: 1,
            ore_cost: 4,
            clay_cost: 2,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 14,
            geode_ore_cost: 2,
            geode_obsidian_cost: 7,
        },
        Blueprint {
            id: 2,
            ore_cost: 2,
            clay_cost: 3,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 8,
            geode_ore_cost: 3,
            geode_obsidian_cost: 12,
        },
    ]);
    assert_eq!(result, 33);
}

fn main() {
    let input = vec![
        Blueprint {
            id: 1,
            ore_cost: 2,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 15,
            geode_ore_cost: 2,
            geode_obsidian_cost: 15,
        },
        Blueprint {
            id: 2,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 12,
            geode_ore_cost: 3,
            geode_obsidian_cost: 8,
        },
        Blueprint {
            id: 3,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 17,
            geode_ore_cost: 4,
            geode_obsidian_cost: 16,
        },
        Blueprint {
            id: 4,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 16,
            geode_ore_cost: 4,
            geode_obsidian_cost: 16,
        },
        Blueprint {
            id: 5,
            ore_cost: 4,
            clay_cost: 3,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 15,
            geode_ore_cost: 3,
            geode_obsidian_cost: 12,
        },
        Blueprint {
            id: 6,
            ore_cost: 2,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 17,
            geode_ore_cost: 3,
            geode_obsidian_cost: 11,
        },
        Blueprint {
            id: 7,
            ore_cost: 3,
            clay_cost: 3,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 19,
            geode_ore_cost: 2,
            geode_obsidian_cost: 9,
        },
        Blueprint {
            id: 8,
            ore_cost: 3,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 18,
            geode_ore_cost: 2,
            geode_obsidian_cost: 11,
        },
        Blueprint {
            id: 9,
            ore_cost: 3,
            clay_cost: 3,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 9,
            geode_ore_cost: 3,
            geode_obsidian_cost: 7,
        },
        Blueprint {
            id: 10,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 15,
            geode_ore_cost: 4,
            geode_obsidian_cost: 17,
        },
        Blueprint {
            id: 11,
            ore_cost: 3,
            clay_cost: 3,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 9,
            geode_ore_cost: 2,
            geode_obsidian_cost: 9,
        },
        Blueprint {
            id: 12,
            ore_cost: 2,
            clay_cost: 4,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 20,
            geode_ore_cost: 2,
            geode_obsidian_cost: 17,
        },
        Blueprint {
            id: 13,
            ore_cost: 4,
            clay_cost: 3,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 8,
            geode_ore_cost: 3,
            geode_obsidian_cost: 7,
        },
        Blueprint {
            id: 14,
            ore_cost: 3,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 20,
            geode_ore_cost: 4,
            geode_obsidian_cost: 16,
        },
        Blueprint {
            id: 15,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 20,
            geode_ore_cost: 2,
            geode_obsidian_cost: 12,
        },
        Blueprint {
            id: 16,
            ore_cost: 4,
            clay_cost: 3,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 13,
            geode_ore_cost: 2,
            geode_obsidian_cost: 10,
        },
        Blueprint {
            id: 17,
            ore_cost: 2,
            clay_cost: 3,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 18,
            geode_ore_cost: 2,
            geode_obsidian_cost: 19,
        },
        Blueprint {
            id: 18,
            ore_cost: 3,
            clay_cost: 4,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 15,
            geode_ore_cost: 3,
            geode_obsidian_cost: 7,
        },
        Blueprint {
            id: 19,
            ore_cost: 3,
            clay_cost: 4,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 10,
            geode_ore_cost: 4,
            geode_obsidian_cost: 8,
        },
        Blueprint {
            id: 20,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 6,
            geode_ore_cost: 2,
            geode_obsidian_cost: 14,
        },
        Blueprint {
            id: 21,
            ore_cost: 2,
            clay_cost: 3,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 16,
            geode_ore_cost: 2,
            geode_obsidian_cost: 11,
        },
        Blueprint {
            id: 22,
            ore_cost: 3,
            clay_cost: 3,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 19,
            geode_ore_cost: 2,
            geode_obsidian_cost: 20,
        },
        Blueprint {
            id: 23,
            ore_cost: 2,
            clay_cost: 3,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 17,
            geode_ore_cost: 3,
            geode_obsidian_cost: 19,
        },
        Blueprint {
            id: 24,
            ore_cost: 3,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 16,
            geode_ore_cost: 3,
            geode_obsidian_cost: 15,
        },
        Blueprint {
            id: 25,
            ore_cost: 2,
            clay_cost: 3,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 17,
            geode_ore_cost: 3,
            geode_obsidian_cost: 10,
        },
        Blueprint {
            id: 26,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 3,
            obsidian_clay_cost: 20,
            geode_ore_cost: 2,
            geode_obsidian_cost: 10,
        },
        Blueprint {
            id: 27,
            ore_cost: 4,
            clay_cost: 4,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 11,
            geode_ore_cost: 2,
            geode_obsidian_cost: 7,
        },
        Blueprint {
            id: 28,
            ore_cost: 4,
            clay_cost: 3,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 10,
            geode_ore_cost: 4,
            geode_obsidian_cost: 10,
        },
        Blueprint {
            id: 29,
            ore_cost: 2,
            clay_cost: 2,
            obsidian_ore_cost: 2,
            obsidian_clay_cost: 8,
            geode_ore_cost: 2,
            geode_obsidian_cost: 14,
        },
        Blueprint {
            id: 30,
            ore_cost: 3,
            clay_cost: 4,
            obsidian_ore_cost: 4,
            obsidian_clay_cost: 18,
            geode_ore_cost: 3,
            geode_obsidian_cost: 8,
        },
    ];

    println!("\n{}", solve1(input));
}
