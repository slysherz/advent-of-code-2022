// Bored of parsing input, and not as fast as I would like >.<

use std::collections::HashMap;

#[derive(Clone)]
struct ValveEntry {
    name: &'static str,
    flow: usize,
    tunnels: &'static str,
}

struct Valve {
    flow: usize,
    tunnels: Vec<usize>,
}

type Key1 = (usize, usize, u64);
type Key2 = (usize, usize, usize, u64);

fn tunnel_list(str: &str) -> Vec<String> {
    str.split(", ").map(|s| s.to_string()).collect()
}

fn valve_list(list: Vec<ValveEntry>) -> (Vec<Valve>, usize) {
    let mut names = HashMap::new();
    for entry in &list {
        names.insert(entry.name.to_string(), names.len());
    }

    let valves = list
        .iter()
        .map(|e| Valve {
            flow: e.flow,
            tunnels: tunnel_list(e.tunnels)
                .iter()
                .map(|t| *names.get(t).unwrap())
                .collect(),
        })
        .collect();

    (valves, *names.get("AA").unwrap())
}

fn merge(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    let mut res = a.clone();
    res.extend(b);
    res
}

fn solve1(
    current: usize,
    valves: &Vec<Valve>,
    minutes_left: usize,
    valves_open: u64,
    cache: &mut HashMap<Key1, usize>,
) -> usize {
    if minutes_left == 0 {
        return 0;
    }

    if let Some(res) = cache.get(&(current, minutes_left, valves_open)) {
        return *res;
    }

    // Try to open the current valve
    let valve = &valves[current];
    let mut result = if valve.flow == 0 || (valves_open >> current) & 1 == 1 {
        0
    } else {
        let temp_valves_open = valves_open | (1 << current);
        valve.flow * (minutes_left - 1)
            + solve1(current, valves, minutes_left - 1, temp_valves_open, cache)
    };

    // Or try to follow a tunnel
    for next in &valves[current].tunnels {
        result = usize::max(
            result,
            solve1(*next, valves, minutes_left - 1, valves_open, cache),
        );
    }

    cache.insert((current, minutes_left, valves_open), result);

    result
}

fn solve2(
    (current1, current2): (usize, usize),
    valves: &Vec<Valve>,
    minutes_left: usize,
    valves_open: u64,
    cache: &mut HashMap<Key2, usize>,
    global_max: &mut usize,
) -> usize {
    if minutes_left == 0 {
        return 0;
    }

    if let Some(res) = cache.get(&(current1, current2, minutes_left, valves_open)) {
        return *res;
    }

    if let Some(res) = cache.get(&(current2, current1, minutes_left, valves_open)) {
        return *res;
    }

    let all_options1 = merge(&valves[current1].tunnels, &[current1].to_vec());
    let all_options2 = merge(&valves[current2].tunnels, &[current2].to_vec());

    let mut result = 0;
    for next1 in all_options1 {
        let mut temp_valves_open1 = valves_open;
        let mut temp_result1 = 0;
        if next1 == current1 {
            // Open current valve unless it's already opened
            if valves[next1].flow == 0 || (temp_valves_open1 >> current1) & 1 == 1 {
                continue;
            }

            temp_valves_open1 ^= 1 << current1;
            temp_result1 += (minutes_left - 1) * valves[current1].flow;
        }

        for &next2 in &all_options2 {
            let mut temp_valves_open2 = temp_valves_open1;
            let mut temp_result2 = temp_result1;
            if next2 == current2 {
                // Open current valve unless it's already opened
                if valves[next2].flow == 0 || (temp_valves_open2 >> current2) & 1 == 1 {
                    continue;
                }

                temp_valves_open2 ^= 1 << current2;
                temp_result2 += (minutes_left - 1) * valves[current2].flow;
            }

            temp_result2 += solve2(
                (next1, next2),
                valves,
                minutes_left - 1,
                temp_valves_open2,
                cache,
                global_max,
            );
            result = usize::max(result, temp_result2);
        }
    }

    cache.insert((current1, current2, minutes_left, valves_open), result);
    if result > *global_max {
        *global_max = result;
        println!("{} ({})", global_max, minutes_left);
    }

    result
}

#[test]
fn example1() {
    let valve_entries: Vec<ValveEntry> = [
        ValveEntry {
            name: "AA",
            flow: 0,
            tunnels: "DD, II, BB",
        },
        ValveEntry {
            name: "BB",
            flow: 13,
            tunnels: "CC, AA",
        },
        ValveEntry {
            name: "CC",
            flow: 2,
            tunnels: "DD, BB",
        },
        ValveEntry {
            name: "DD",
            flow: 20,
            tunnels: "CC, AA, EE",
        },
        ValveEntry {
            name: "EE",
            flow: 3,
            tunnels: "FF, DD",
        },
        ValveEntry {
            name: "FF",
            flow: 0,
            tunnels: "EE, GG",
        },
        ValveEntry {
            name: "GG",
            flow: 0,
            tunnels: "FF, HH",
        },
        ValveEntry {
            name: "HH",
            flow: 22,
            tunnels: "GG",
        },
        ValveEntry {
            name: "II",
            flow: 0,
            tunnels: "AA, JJ",
        },
        ValveEntry {
            name: "JJ",
            flow: 21,
            tunnels: "II",
        },
    ]
    .to_vec();

    let mut cache = HashMap::new();
    let (valves, start) = valve_list(valve_entries);
    let result = solve1(start, &valves, 30, 0, &mut cache);
    assert_eq!(result, 1651);
}

fn main() {
    let valve_entries: Vec<ValveEntry> = [
        ValveEntry {
            name: "TM",
            flow: 3,
            tunnels: "GU, KQ, BV, MK",
        },
        ValveEntry {
            name: "BX",
            flow: 0,
            tunnels: "CD, HX",
        },
        ValveEntry {
            name: "GV",
            flow: 8,
            tunnels: "MP, SE",
        },
        ValveEntry {
            name: "OI",
            flow: 0,
            tunnels: "ZB, RG",
        },
        ValveEntry {
            name: "OY",
            flow: 0,
            tunnels: "XG, ZB",
        },
        ValveEntry {
            name: "EZ",
            flow: 0,
            tunnels: "OU, LI",
        },
        ValveEntry {
            name: "TN",
            flow: 0,
            tunnels: "DT, GU",
        },
        ValveEntry {
            name: "SE",
            flow: 0,
            tunnels: "GV, CD",
        },
        ValveEntry {
            name: "SG",
            flow: 0,
            tunnels: "XR, NK",
        },
        ValveEntry {
            name: "EB",
            flow: 0,
            tunnels: "SJ, CE",
        },
        ValveEntry {
            name: "QB",
            flow: 0,
            tunnels: "AW, MI",
        },
        ValveEntry {
            name: "GU",
            flow: 0,
            tunnels: "TN, TM",
        },
        ValveEntry {
            name: "AW",
            flow: 11,
            tunnels: "QB, IG, IK, VK",
        },
        ValveEntry {
            name: "IG",
            flow: 0,
            tunnels: "AW, SH",
        },
        ValveEntry {
            name: "MJ",
            flow: 0,
            tunnels: "IK, XR",
        },
        ValveEntry {
            name: "HX",
            flow: 0,
            tunnels: "BX, AA",
        },
        ValveEntry {
            name: "IK",
            flow: 0,
            tunnels: "MJ, AW",
        },
        ValveEntry {
            name: "QZ",
            flow: 0,
            tunnels: "AF, XG",
        },
        ValveEntry {
            name: "CV",
            flow: 0,
            tunnels: "KT, AA",
        },
        ValveEntry {
            name: "ES",
            flow: 0,
            tunnels: "BV, CD",
        },
        ValveEntry {
            name: "NK",
            flow: 0,
            tunnels: "YQ, SG",
        },
        ValveEntry {
            name: "SL",
            flow: 0,
            tunnels: "DT, XL",
        },
        ValveEntry {
            name: "RG",
            flow: 17,
            tunnels: "SJ, OI, WC",
        },
        ValveEntry {
            name: "ZB",
            flow: 9,
            tunnels: "OY, MP, DI, OX, OI",
        },
        ValveEntry {
            name: "SJ",
            flow: 0,
            tunnels: "RG, EB",
        },
        ValveEntry {
            name: "GF",
            flow: 19,
            tunnels: "DQ, SH, IH",
        },
        ValveEntry {
            name: "OU",
            flow: 10,
            tunnels: "EZ, TL, WC",
        },
        ValveEntry {
            name: "TL",
            flow: 0,
            tunnels: "OU, OX",
        },
        ValveEntry {
            name: "XG",
            flow: 18,
            tunnels: "QZ, OY",
        },
        ValveEntry {
            name: "EK",
            flow: 20,
            tunnels: "FD, MI",
        },
        ValveEntry {
            name: "BV",
            flow: 0,
            tunnels: "TM, ES",
        },
        ValveEntry {
            name: "AA",
            flow: 0,
            tunnels: "CV, HX, TR, MK, DQ",
        },
        ValveEntry {
            name: "UO",
            flow: 23,
            tunnels: "AF",
        },
        ValveEntry {
            name: "LI",
            flow: 0,
            tunnels: "EZ, CE",
        },
        ValveEntry {
            name: "MI",
            flow: 0,
            tunnels: "EK, QB",
        },
        ValveEntry {
            name: "MP",
            flow: 0,
            tunnels: "GV, ZB",
        },
        ValveEntry {
            name: "YQ",
            flow: 14,
            tunnels: "VK, MG, NK",
        },
        ValveEntry {
            name: "AF",
            flow: 0,
            tunnels: "UO, QZ",
        },
        ValveEntry {
            name: "SH",
            flow: 0,
            tunnels: "IG, GF",
        },
        ValveEntry {
            name: "FD",
            flow: 0,
            tunnels: "IH, EK",
        },
        ValveEntry {
            name: "KQ",
            flow: 0,
            tunnels: "TM, FQ",
        },
        ValveEntry {
            name: "DI",
            flow: 0,
            tunnels: "ZB, CD",
        },
        ValveEntry {
            name: "KT",
            flow: 0,
            tunnels: "DT, CV",
        },
        ValveEntry {
            name: "MG",
            flow: 0,
            tunnels: "NQ, YQ",
        },
        ValveEntry {
            name: "DQ",
            flow: 0,
            tunnels: "GF, AA",
        },
        ValveEntry {
            name: "CE",
            flow: 21,
            tunnels: "LI, EB",
        },
        ValveEntry {
            name: "MK",
            flow: 0,
            tunnels: "AA, TM",
        },
        ValveEntry {
            name: "XL",
            flow: 0,
            tunnels: "CD, SL",
        },
        ValveEntry {
            name: "OX",
            flow: 0,
            tunnels: "TL, ZB",
        },
        ValveEntry {
            name: "DT",
            flow: 5,
            tunnels: "NQ, TP, KT, SL, TN",
        },
        ValveEntry {
            name: "IH",
            flow: 0,
            tunnels: "GF, FD",
        },
        ValveEntry {
            name: "TP",
            flow: 0,
            tunnels: "XR, DT",
        },
        ValveEntry {
            name: "FQ",
            flow: 0,
            tunnels: "XR, KQ",
        },
        ValveEntry {
            name: "CD",
            flow: 6,
            tunnels: "DI, BX, XL, ES, SE",
        },
        ValveEntry {
            name: "XR",
            flow: 7,
            tunnels: "TR, FQ, TP, MJ, SG",
        },
        ValveEntry {
            name: "VK",
            flow: 0,
            tunnels: "YQ, AW",
        },
        ValveEntry {
            name: "WC",
            flow: 0,
            tunnels: "RG, OU",
        },
        ValveEntry {
            name: "TR",
            flow: 0,
            tunnels: "XR, AA",
        },
        ValveEntry {
            name: "NQ",
            flow: 0,
            tunnels: "DT, MG",
        },
    ]
    .to_vec();

    let mut cache = HashMap::new();
    let mut global_max = 0;
    let (valves, start) = valve_list(valve_entries);
    let result = solve2((start, start), &valves, 26, 0, &mut cache, &mut global_max);
    println!("{}", result);
}
