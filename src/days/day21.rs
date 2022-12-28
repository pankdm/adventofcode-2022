// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn read_main_input() -> Vec<String> {
    let args = std::env::args().cv();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    let path = format!("input/day21/{}", file);
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
        assert_eq!(part1(&lines), -1);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), -1);
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn dp0(
    s: &String,
    cache: &mut HashMap<String, i64>,
    monkeys: &HashMap<String, Vec<String>>,
) -> (i64, bool) {
    if cache.contains_key(s) {
        return (cache[s], true);
    }

    let prev = &monkeys[s];
    let mut result = 0;
    if prev.len() == 1 {
        result = prev[0].to_i64();
    } else {
        let op = &prev[1];
        let (a, ok_a) = dp0(&prev[0], cache, monkeys);
        let (b, ok_b) = dp0(&prev[2], cache, monkeys);
        if !ok_a || !ok_b {
            println!("not ok at s={}, {} {}", s, ok_a, ok_b);
            return (-1, false);
        }
        result = match op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => unreachable!(),
        };
    }
    cache.insert(s.clone(), result);
    (result, true)
}

pub fn dp(
    s: &String,
    cache: &mut HashMap<String, i64>,
    monkeys: &HashMap<String, Vec<String>>,
) -> (i64, bool) {
    if s == "humn" {
        return (-1, false);
    }
    if cache.contains_key(s) {
        return (cache[s], true);
    }

    let prev = &monkeys[s];
    let mut result = 0;
    if prev.len() == 1 {
        result = prev[0].to_i64();
    } else {
        let op = &prev[1];
        let (a, ok_a) = dp(&prev[0], cache, monkeys);
        let (b, ok_b) = dp(&prev[2], cache, monkeys);
        if !ok_a || !ok_b {
            return (-1, false);
        }
        result = match op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => unreachable!(),
        };
    }
    cache.insert(s.clone(), result);
    (result, true)
}

pub fn find(
    s: &String,
    eq: i64,
    cache: &mut HashMap<String, i64>,
    monkeys: &HashMap<String, Vec<String>>,
) -> i64 {
    if s == "humn" {
        return eq;
    }
    // println!("find at {}", s);
    let prev = &monkeys[s];
    let op = &prev[1];
    let (a, ok_a) = dp(&prev[0], cache, monkeys);
    let (b, ok_b) = dp(&prev[2], cache, monkeys);

    assert!(ok_a || ok_b);
    if !ok_a {
        assert!(ok_b);
        // a ? b == eq
        let a = match op.as_str() {
            "+" => eq - b,
            "-" => eq + b,
            "*" => eq / b,
            "/" => eq * b,
            _ => unreachable!(),
        };
        let new_eq = a;
        return find(&prev[0], new_eq, cache, monkeys);
    } else if !ok_b {
        assert!(ok_a);
        // a ? b == eq
        let b = match op.as_str() {
            "+" => eq - a,
            "-" => a - eq,
            "*" => eq / a,
            "/" => a / eq,
            _ => unreachable!(),
        };
        let new_eq = b;
        return find(&prev[2], new_eq, cache, monkeys);
    } else {
    }
    unreachable!();
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut monkeys = HashMap::new();

    for line in lines {
        let p = split_string(line, ": ");
        let name = p[0].clone();
        // println!("{:?}", p);
        let ops = split_string(&p[1], " ");
        monkeys.insert(name, ops);
    }
    let mut cache = HashMap::new();
    let res = dp0(&"root".to_string(), &mut cache, &monkeys);
    println!("res = {:?}", res);
    res.0
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut monkeys = HashMap::new();

    let mut counts: HashMap<String, i64> = HashMap::new();

    for line in lines {
        let p = split_string(line, ": ");
        let name = p[0].clone();
        // println!("{:?}", p);
        let ops = split_string(&p[1], " ");
        if ops.len() > 1 {
            *counts.entry(ops[0].clone()).or_default() += 1;
            *counts.entry(ops[2].clone()).or_default() += 1;
        }
        monkeys.insert(name, ops);
    }
    let mut cache = HashMap::new();

    let left = &monkeys["root"][0];
    let right = &monkeys["root"][2];

    for (k, v) in counts.iter() {
        if *v > 1 {
            println!("{} {}", k, v);
        }
    }

    let a = dp(left, &mut cache, &monkeys);
    let b = dp(right, &mut cache, &monkeys);
    println!("{:?} != {:?}", a, b);
    let res = find(left, b.0, &mut cache, &monkeys);
    res
}
