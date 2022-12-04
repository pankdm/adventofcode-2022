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
    let path = format!("input/day04/{}", file);
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

pub fn parse_pair(line: &str) -> (i64, i64) {
    let parts = split_string(line, "-");
    (parts[0].to_i64(), parts[1].to_i64())
}
pub fn is_within(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

pub fn is_overlap(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.0 && b.0 <= a.1
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for line in lines {
        let p = split_string(line, ",");
        let a = parse_pair(&p[0]);
        let b = parse_pair(&p[1]);
        if is_within(a, b) || is_within(b, a) {
            res += 1;
        }
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for line in lines {
        let p = split_string(line, ",");
        let a = parse_pair(&p[0]);
        let b = parse_pair(&p[1]);
        if is_overlap(a, b) || is_overlap(b, a) {
            res += 1;
        }
    }
    res
}
