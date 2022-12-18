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
    let path = format!("input/day18/{}", file);
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

fn adjacent_cubes(c0: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut res = Vec::new();
    for i in 0..3 {
        for d in vec![-1, 1] {
            let mut c1 = c0.clone();
            c1[i] += d;
            res.push(c1);
        }
    }
    res
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut cubes = HashMap::new();

    for line in lines {
        let p = split_string(line, ",").iter().map(|x| x.to_i64()).cv();
        cubes.insert(p, 1);
    }
    let mut exposed = 0;
    for c0 in cubes.keys() {
        for c1 in adjacent_cubes(c0) {
            if !cubes.contains_key(&c1) {
                exposed += 1
            }
        }
    }
    exposed
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut cubes = HashMap::new();

    for line in lines {
        let p = split_string(line, ",").iter().map(|x| x.to_i64()).cv();
        cubes.insert(p, 1);
    }
    let mut bmin = vec![0; 3];
    let mut bmax = vec![0; 3];
    for i in 0..3 {
        bmin[i] = cubes.keys().map(|p| p[i]).min().unwrap() - 1;
        bmax[i] = cubes.keys().map(|p| p[i]).max().unwrap() + 1;
    }

    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(bmin.clone());
    visited.insert(bmin.clone(), true);
    loop {
        if q.is_empty() {
            break;
        }
        let now = q.pop_front().unwrap();
        for next in adjacent_cubes(&now) {
            if (0..3).any(|i| next[i] < bmin[i] || next[i] > bmax[i]) {
                continue;
            }
            if cubes.contains_key(&next) {
                continue;
            }
            if visited.contains_key(&next) {
                continue;
            }
            visited.insert(next.clone(), true);
            q.push_back(next);
        }
    }

    let mut exterior = 0;
    for c0 in cubes.keys() {
        for c1 in adjacent_cubes(c0) {
            if visited.contains_key(&c1) {
                exterior += 1;
            }
        }
    }
    exterior
}
