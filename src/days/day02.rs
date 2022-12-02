// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
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
    let path = format!("input/day02/{}", file);
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

pub fn cost(s: &str) -> i64 {
    if s == "A" || s == "X" {
        return 1;
    }
    if s == "B" || s == "Y" {
        return 2;
    }
    return 3;
}

pub fn get_score(s: &str) -> i64 {
    if s == "X" {
        return 0;
    }
    if s == "Y" {
        return 3;
    }
    return 6;
}

// Rock < Paper < Scissor
pub fn outcome(a: i64, b: i64) -> i64 {
    if a == b {
        return 3;
    }
    if a == 1 {
        if b == 2 {
            return 6;
        }
        if b == 3 {
            return 0;
        }
    }
    if a == 2 {
        if b == 1 {
            return 0;
        }
        if b == 3 {
            return 6;
        }
    }
    if a == 3 {
        if b == 2 {
            return 0;
        }
        if b == 1 {
            return 6;
        }
    }
    unreachable!();
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut total = 0;
    for line in lines {
        let parts = split_string(line, " ");
        let a = cost(&parts[0]);
        let b = cost(&parts[1]);
        let mut score = b + outcome(a, b);
        total += score;
    }
    total
}

pub fn find_move(a: i64, score: i64) -> i64 {
    for b in 1..=3 {
        if outcome(a, b) == score {
            return b;
        }
    }
    unreachable!();
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut total = 0;
    for line in lines {
        let parts = split_string(line, " ");
        let a = cost(&parts[0]);
        let desired = get_score(&parts[1]);
        let b = find_move(a, desired);
        let mut score = b + outcome(a, b);
        total += score;
    }
    total
}
