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
    let path = format!("input/day12/{}", file);
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

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn dfs_from(start: (i64, i64), end: (i64, i64), grid: &Vec<Vec<i64>>) -> i64 {
    let mut q = VecDeque::new();
    let mut visited = HashMap::new();

    q.push_back((start, 0));
    visited.insert(start, 0);
    loop {
        if q.is_empty() {
            break;
        }
        let (cur, steps) = q.pop_front().unwrap();
        if cur == end {
            break;
        }
        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_x = cur.0 + dx;
            let next_y = cur.1 + dy;
            if 0 <= next_x
                && next_x < grid.len() as i64
                && 0 <= next_y
                && next_y < grid[0].len() as i64
            {
                let next_value = grid[next_x as usize][next_y as usize];
                let cur_value = grid[cur.0 as usize][cur.1 as usize];
                if next_value <= cur_value + 1 {
                    let next = (next_x, next_y);
                    if !visited.contains_key(&next) {
                        q.push_back((next, steps + 1));
                        visited.insert(next, steps + 1);
                    }
                }
            }
        }
    }
    if visited.contains_key(&end) {
        return visited[&end];
    } else {
        return -1;
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row_idx, line) in lines.iter().enumerate() {
        let chars = line.to_vec();
        let mut row = Vec::new();
        for i in 0..chars.len() {
            let ch = match chars[i] {
                'S' => {
                    start = (row_idx as i64, i as i64);
                    'a'
                }
                'E' => {
                    end = (row_idx as i64, i as i64);
                    'z'
                }
                _ => chars[i],
            };
            row.push((ch as u8 - 'a' as u8) as i64)
        }
        grid.push(row)
    }
    println!("start = {:?}, end = {:?}", start, end);
    dfs_from(start, end, &grid)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();

    let mut starts = Vec::new();
    let mut end = (0, 0);

    for (row_idx, line) in lines.iter().enumerate() {
        let chars = line.to_vec();
        let mut row = Vec::new();
        for i in 0..chars.len() {
            let coords = (row_idx as i64, i as i64);
            let ch = match chars[i] {
                'S' => 'a',
                'E' => {
                    end = coords;
                    'z'
                }
                _ => chars[i],
            };
            if ch == 'a' {
                starts.push(coords);
            }
            row.push((ch as u8 - 'a' as u8) as i64)
        }
        grid.push(row)
    }
    starts
        .iter()
        .map(|x| dfs_from(*x, end, &grid))
        .filter(|x| *x != -1)
        .min()
        .unwrap()
}
