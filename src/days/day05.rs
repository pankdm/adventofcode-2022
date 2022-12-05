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
    let path = format!("input/day05/{}", file);
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

pub fn parts_impl(lines: &Vec<String>, is_part2: bool) -> i64 {
    let mut index = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for i in 0..9 {
        stacks.push(Vec::new());
    }

    loop {
        let line = lines[index].clone().to_vec();
        if line[1] == '1' {
            break;
        }
        index += 1;
        for i in 0..9 {
            let c = line[1 + i * 4];
            // println!("{} -> {}", i, c);
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    for i in 0..9 {
        stacks[i].reverse();
        println!("{} {:?}", i, stacks[i]);
    }

    // empty line
    index += 1;
    index += 1;
    loop {
        if index >= lines.len() {
            break;
        }
        let line = lines[index].clone();
        let parts = split_string(&line, " ");
        let count = parts[1].to_i64();
        let from = parts[3].to_i64() as usize - 1;
        let to = parts[5].to_i64() as usize - 1;
        let mut tmp = Vec::new();
        for i in 0..count {
            let c = stacks[from]
                .pop()
                .expect(&format!("error at line '{}' (i = {})", line, i));
            tmp.push(c);
        }

        if is_part2 {
            tmp.reverse();
        }
        for c in tmp {
            stacks[to].push(c);
        }

        index += 1;
    }
    let mut res = Vec::new();
    for i in 0..9 {
        res.push(stacks[i].last().unwrap().clone());
    }
    println!("{}", res.to_str());
    -1
}

pub fn part1(lines: &Vec<String>) -> i64 {
    parts_impl(lines, false);
    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    parts_impl(lines, true);
    -1
}
