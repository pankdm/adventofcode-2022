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
    let path = format!("input/day06/{}", file);
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

pub fn parts_impl(lines: &Vec<String>, chars: usize) -> i64 {
    let line = lines[0].to_vec();
    for i in (chars - 1)..line.len() {
        let mut s = HashSet::new();
        for j in i + 1 - chars..=i {
            s.insert(line[j]);
        }
        if s.len() == chars {
            return (i + 1) as i64;
        }
    }
    -1
}

pub fn part1(lines: &Vec<String>) -> i64 {
    parts_impl(lines, 4)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    parts_impl(lines, 14)
}
