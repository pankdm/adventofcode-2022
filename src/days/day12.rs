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

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn bfs_from(start: Vec2, end: Vec2, grid: &Vec<Vec<i64>>) -> i64 {
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
        for d in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = cur + Vec2::from_tuple(d);
            if grid.inside(next) {
                let next_value = grid.get(next);
                let cur_value = grid.get(cur);
                if next_value <= cur_value + 1 {
                    if !visited.contains_key(&next) {
                        q.push_back((next, steps + 1));
                        visited.insert(next, steps + 1);
                    }
                }
            }
        }
    }
    if visited.contains_key(&end) {
        visited[&end]
    } else {
        -1
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();

    let mut start = Vec2::new(0, 0);
    let mut end = Vec2::new(0, 0);

    for (y, line) in lines.iter().enumerate() {
        let chars = line.cv();
        let mut row = Vec::new();
        for x in 0..chars.len() {
            let coords = Vec2::new(x as i64, y as i64);
            let ch = match chars[x] {
                'S' => {
                    start = coords;
                    'a'
                }
                'E' => {
                    end = coords;
                    'z'
                }
                _ => chars[x],
            };
            row.push((ch as u8 - 'a' as u8) as i64)
        }
        grid.push(row)
    }
    bfs_from(start, end, &grid)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();

    let mut starts = Vec::new();
    let mut end = Vec2::new(0, 0);

    for (y, line) in lines.iter().enumerate() {
        let chars = line.cv();
        let mut row = Vec::new();
        for x in 0..chars.len() {
            let coords = Vec2::new(x as i64, y as i64);
            let ch = match chars[x] {
                'S' => 'a',
                'E' => {
                    end = coords;
                    'z'
                }
                _ => chars[x],
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
        .map(|x| bfs_from(*x, end, &grid))
        .filter(|x| *x != -1)
        .min()
        .unwrap()
}
