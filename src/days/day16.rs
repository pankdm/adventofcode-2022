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
        // assert_eq!(part2(&lines), -1);
    }
}

pub fn main() {
    let lines = read_main_input();

    // println!("part1 = {}", part1(&lines));
    println!("part1 = {}", part_impl(&lines, true));
    println!("part2 = {}", part_impl(&lines, false));
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    cur: i64,
    opened: i64,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cur.hash(state);
        self.opened.hash(state);
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

pub fn set_bit(value: i64, bit: i64) -> i64 {
    value | (1 << bit)
}

pub fn has_bit(value: i64, bit: i64) -> bool {
    (value & (1 << bit)) > 0
}

fn run_dijkstra(
    max_steps: i64,
    start_pos: &Vec<(State, i64)>,
    fast_graph: &HashMap<i64, HashMap<i64, i64>>,
    costs: &HashMap<i64, i64>,
) -> HashMap<State, i64> {
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    for (s, score) in start_pos {
        q.push_back((s.clone(), 0, *score));
        visited.insert(s.clone(), *score);
    }

    let mut best = 0;
    let mut counter = 0;
    loop {
        if q.is_empty() {
            break;
        }
        counter += 1;
        let (now, elapsed, score) = q.pop_front().unwrap();
        if score > best {
            best = best.max(score);
            println!("best = {}, counter = {}", best, counter);
        }

        if visited.contains_key(&now) {
            let prev_score = visited[&now];
            if prev_score > score {
                continue;
            }
        }
        visited.insert(now.clone(), score);
        if elapsed == max_steps {
            continue;
        }

        for (next_v, time) in fast_graph[&now.cur].iter() {
            if has_bit(now.opened, *next_v) {
                continue;
            }
            assert!(costs[next_v] > 0);
            let mut next = now.clone();
            next.cur = next_v.clone();
            next.opened = set_bit(next.opened, *next_v);
            let open_ts = elapsed + time + 1;
            if open_ts < max_steps {
                q.push_back((next, open_ts, score + (max_steps - open_ts) * costs[next_v]))
            }
        }
    }
    visited
}

pub fn part_impl(lines: &Vec<String>, part1: bool) -> i64 {
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

    if part1 {
        let s = State { cur: 0, opened: 0 };
        let start_pos = vec![(s, 0)];
        let visited = run_dijkstra(30, &start_pos, &fast_graph, &costs);
        let best = visited.values().max().unwrap();
        *best
    } else {
        let s = State { cur: 0, opened: 0 };
        let start_pos = vec![(s, 0)];
        let visited = run_dijkstra(26, &start_pos, &fast_graph, &costs);
        println!("count = {}", visited.len());

        let mut start_pos2 = Vec::new();
        for (s, score) in visited.iter() {
            let mut next = s.clone();
            next.cur = 0;
            start_pos2.push((next, *score));
        }
        let visited2 = run_dijkstra(26, &start_pos2, &fast_graph, &costs);
        let best = visited2.values().max().unwrap();
        *best
    }
}
