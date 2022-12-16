// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use crate::*;

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().cv();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    let path = format!("input/day16/{}", file);
    read_input(&path)
    // read_input_as_string(&path)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        // assert_eq!(part1(&lines), -1);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), -1);
    }
}

pub fn main() {
    let lines = read_main_input();

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    cur: i64,
    opened: HashSet<i64>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cur.hash(state);
        for x in self.opened.iter().sorted() {
            x.hash(state);
        }
    }
}

type Graph = HashMap<String, Vec<String>>;

pub fn bfs(start: &String, graph: &Graph) -> HashMap<String, i64> {
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start.clone(), 0));
    visited.insert(start.clone(), 0);

    loop {
        if q.is_empty() {
            break;
        }
        let (now, time) = q.pop_front().unwrap();
        for next in graph[&now].iter() {
            if visited.contains_key(next) {
                continue;
            }
            q.push_back((next.clone(), time + 1));
            visited.insert(next.to_string(), time + 1);
        }
    }
    visited
}

// pub fn part1(lines: &Vec<String>) -> i64 {
//     let mut graph = HashMap::new();
//     let mut costs = HashMap::new();

//     let mut non_zero = 0;
//     for line in lines {
//         let re =
//             Regex::new(r"Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
//         let cap = re.captures(line).unwrap();
//         let src = cap[1].to_string();
//         let flow = cap[2].to_i64();
//         let dst = split_string(&cap[3], ", ");
//         println!("{} {:?} {}", src, dst, flow);
//         graph.insert(src.clone(), dst);
//         costs.insert(src, flow);
//         if flow > 0 {
//             non_zero += 1;
//         }
//         // break;
//     }

//     let mut fast_graph: HashMap<String, HashMap<String, i64>> = HashMap::new();
//     for start in graph.keys() {
//         let visited = bfs(start, &graph);
//         let mut dsts = HashMap::new();
//         for (k, v) in visited.iter() {
//             if costs[k] > 0 {
//                 dsts.insert(k.clone(), *v);
//             }
//         }
//         fast_graph.insert(start.clone(), dsts);
//     }

//     println!("non zero = {}", non_zero);

//     let mut q = VecDeque::new();
//     let mut visited = HashMap::new();
//     let s = State {
//         cur: "AA".to_string(),
//         opened: Vec::new(),
//     };
//     q.push_back((s.clone(), 0, 0));
//     visited.insert(s, 0);

//     let mut best = 0;
//     let mut counter = 0;
//     loop {
//         if q.is_empty() {
//             break;
//         }
//         counter += 1;
//         let (now, elapsed, score) = q.pop_front().unwrap();
//         // println!(
//         //     "   At state = {:?}, elapsed = {}, score = {}",
//         //     now, elapsed, score
//         // );
//         // if counter > 3 {
//         //     break;
//         // }
//         // if visited.contains_key(&now) {
//         //     let prev_score = visited[&now];
//         //     if prev_score < score {
//         //         continue;
//         //     }
//         // }
//         if score > best {
//             best = best.max(score);
//             println!("best = {}, counter = {}", best, counter);
//         }

//         // visited.insert(now.clone(), score);
//         if elapsed == 30 {
//             continue;
//         }

//         for (next_v, time) in fast_graph[&now.cur].iter() {
//             if now.opened.contains(next_v) {
//                 continue;
//             }
//             assert!(costs[next_v] > 0);
//             let mut next = now.clone();
//             next.cur = next_v.clone();
//             next.opened.push(next_v.clone());
//             let open_ts = elapsed + time + 1;
//             if open_ts < 30 {
//                 q.push_back((next, open_ts, score + (30 - open_ts) * costs[next_v]))
//             }
//         }
//     }
//     best
// }

pub fn set_bit(value: i64, bit: i64) -> i64 {
    value | (1 << bit)
}

pub fn has_bit(value: i64, bit: i64) -> bool {
    (value & (1 << bit)) > 0
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut graph = HashMap::new();
    let mut costs = HashMap::new();

    let mut index = 0;
    let mut mapping: HashMap<String, i64> = HashMap::new();
    mapping.insert("AA".to_string(), index);
    index += 1;

    let mut non_zero = HashSet::new();
    for line in lines {
        let re =
            Regex::new(r"Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        let cap = re.captures(line).unwrap();
        let src = cap[1].to_string();
        let flow = cap[2].to_i64();
        let dst = split_string(&cap[3], ", ");
        // println!("{} {:?} {}", src, dst, flow);
        graph.insert(src.clone(), dst);
        if flow > 0 {
            non_zero.insert(src.clone());

            mapping.insert(src.to_string(), index);
            costs.insert(index, flow);
            index += 1;
        }
        // break;
    }

    let mut fast_graph: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    for start in graph.keys() {
        let visited = bfs(start, &graph);
        let mut dsts = HashMap::new();
        for (k, v) in visited.iter() {
            if non_zero.contains(k) {
                dsts.insert(mapping[k], *v);
            }
        }
        if mapping.contains_key(start) {
            fast_graph.insert(mapping[start], dsts);
        }
    }

    println!("non zero = {}", non_zero.len());

    let mut visited = HashMap::new();
    let s = State {
        cur: 0,
        opened: HashSet::new(),
    };
    visited.insert(s.clone(), 0);

    {
        let mut q = VecDeque::new();
        q.push_back((s.clone(), 0, 0));

        let mut best = 0;
        let mut counter = 0;
        loop {
            if q.is_empty() {
                break;
            }
            counter += 1;
            let (now, elapsed, score) = q.pop_front().unwrap();
            // println!(
            //     "   At state = {:?}, elapsed = {}, score = {}",
            //     now, elapsed, score
            // );
            // if counter > 3 {
            //     break;
            // }
            // if visited.contains_key(&now) {
            //     let prev_score = visited[&now];
            //     if prev_score < score {
            //         continue;
            //     }
            // }
            if score > best {
                best = best.max(score);
                println!("best = {}, counter = {}", best, counter);
            }
            if visited.contains_key(&now) {
                let before = visited[&now];
                if score > before {
                    visited.insert(now.clone(), score);
                }
            } else {
                visited.insert(now.clone(), score);
            }

            // visited.insert(now.clone(), score);
            if elapsed == 26 {
                continue;
            }

            for (next_v, time) in fast_graph[&now.cur].iter() {
                // if has_bit(now.opened, *next_v) {
                //     continue;
                // }
                if now.opened.contains(next_v) {
                    continue;
                }
                assert!(costs[next_v] > 0);
                let mut next = now.clone();
                next.cur = next_v.clone();
                // next.opened = set_bit(now.opened, *next_v);
                next.opened.insert(*next_v);
                // next.opened.sort();
                let open_ts = elapsed + time + 1;
                if open_ts < 26 {
                    q.push_back((next, open_ts, score + (26 - open_ts) * costs[next_v]))
                }
            }
        }
    }
    println!("count = {}", visited.len());

    let mut q2 = VecDeque::new();
    let mut visited2 = HashMap::new();
    for (s, score) in visited.iter() {
        let mut next = s.clone();
        next.cur = 0;
        q2.push_back((next.clone(), 0, *score));
        visited2.insert(next, *score);
    }

    let mut best2 = 0;
    let mut counter2 = 0;
    loop {
        if q2.is_empty() {
            break;
        }
        counter2 += 1;
        let (now, elapsed, score) = q2.pop_front().unwrap();
        // println!(
        //     "   At state = {:?}, elapsed = {}, score = {}",
        //     now, elapsed, score
        // );
        // if counter > 3 {
        //     break;
        // }
        if visited2.contains_key(&now) {
            let prev_score = visited2[&now];
            if prev_score > score {
                continue;
            }
        }
        visited2.insert(now.clone(), score);

        if score > best2 {
            best2 = best2.max(score);
            println!("... best = {}, counter = {}", best2, counter2);
        }
        // if visited.contains_key(&now) {
        //     let before = visited[&now];
        //     if score > before {
        //         visited.insert(now.clone(), score);
        //     }
        // } else {
        //     visited.insert(now.clone(), score);
        // }

        // visited.insert(now.clone(), score);
        if elapsed == 26 {
            continue;
        }

        for (next_v, time) in fast_graph[&now.cur].iter() {
            // if has_bit(now.opened, *next_v) {
            //     continue;
            // }
            if now.opened.contains(next_v) {
                continue;
            }
            assert!(costs[next_v] > 0);
            let mut next = now.clone();
            next.cur = next_v.clone();
            // next.opened = set_bit(now.opened, *next_v);
            next.opened.insert(*next_v);
            // next.opened.sort();
            let open_ts = elapsed + time + 1;
            if open_ts < 26 {
                q2.push_back((next, open_ts, score + (26 - open_ts) * costs[next_v]))
            }
        }
    }

    best2
}
