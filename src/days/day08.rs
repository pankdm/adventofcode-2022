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
    let path = format!("input/day08/{}", file);
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

fn is_visible(grid: &Vec<Vec<i64>>, r0: i64, c0: i64, dir: (i64, i64)) -> bool {
    let compare = grid[r0 as usize][c0 as usize];
    let (dr, dc) = dir;
    let mut r = r0 + dr;
    let mut c = c0 + dc;
    loop {
        if r < 0 || c < 0 || r >= grid.len() as i64 || c >= grid[r as usize].len() as i64 {
            return true;
        }
        if grid[r as usize][c as usize] >= compare {
            return false;
        }
        r += dr;
        c += dc;
    }
}

fn scenic_score(grid: &Vec<Vec<i64>>, r0: i64, c0: i64, dir: (i64, i64)) -> i64 {
    let compare = grid[r0 as usize][c0 as usize];
    let (dr, dc) = dir;
    let mut r = r0 + dr;
    let mut c = c0 + dc;
    let mut counter = 0;
    loop {
        if r < 0 || c < 0 || r >= grid.len() as i64 || c >= grid[r as usize].len() as i64 {
            return counter;
        }
        if grid[r as usize][c as usize] >= compare {
            return counter + 1;
        }
        r += dr;
        c += dc;
        counter += 1;
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();
    for line in lines {
        let row = line
            .to_vec()
            .iter()
            .map(|c| (*c as u8 - '0' as u8) as i64)
            .cv();
        grid.push(row);
    }
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut res = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let mut ok = false;
            for dir in dirs.iter() {
                if is_visible(&grid, r as i64, c as i64, dir.clone()) {
                    ok = true;
                    break;
                }
            }
            if ok {
                res += 1;
            }
        }
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();
    for line in lines {
        let row = line
            .to_vec()
            .iter()
            .map(|c| (*c as u8 - '0' as u8) as i64)
            .cv();
        grid.push(row);
    }
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut res = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let mut score = 1;
            for dir in dirs.iter() {
                score *= scenic_score(&grid, r as i64, c as i64, dir.clone());
            }
            res = res.max(score);
        }
    }
    res
}
