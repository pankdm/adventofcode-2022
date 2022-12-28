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
    let path = format!("input/day09/{}", file);
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

#[derive(Debug)]
struct Rope {
    head: Vec2,
    tail: Vec2,
}

fn vec_abs(v: Vec2) -> i64 {
    v.x.abs() + v.y.abs()
}

impl Rope {
    fn update(&mut self) {
        let v = self.head + (self.tail * -1);
        let abs = v.x.abs() + v.y.abs();
        if abs <= 1 {
            return;
        }
        if v.x.abs() == 0 || v.y.abs() == 0 {
            self.tail = self.tail + Vec2::new(v.x.signum(), v.y.signum());
        } else {
            if abs == 2 {
                return;
            }
            self.tail = self.tail + Vec2::new(v.x.signum(), v.y.signum());
        }
    }

    fn step(&mut self, dir: Vec2) {
        self.head = self.head + dir;
        self.update();
    }
}

struct BigRope {
    knots: Vec<Vec2>,
}

impl BigRope {
    fn step(&mut self, dir: Vec2) {
        self.knots[0] = self.knots[0] + dir;
        self.update_all();
    }

    fn update_all(&mut self) {
        for i in 0..self.knots.len() - 1 {
            self.update(i, i + 1);
        }
    }

    fn update(&mut self, a: usize, b: usize) {
        let head = self.knots[a];
        let tail = self.knots[b];
        let v = head + (tail * -1);
        let abs = v.x.abs() + v.y.abs();
        if abs <= 1 {
            return;
        }
        if v.x.abs() == 0 || v.y.abs() == 0 {
            self.knots[b] = tail + Vec2::new(v.x.signum(), v.y.signum());
        } else {
            if abs == 2 {
                return;
            }
            self.knots[b] = tail + Vec2::new(v.x.signum(), v.y.signum());
        }
    }
}

pub fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut rope = Rope {
        head: Vec2::new(0, 0),
        tail: Vec2::new(0, 0),
    };
    let mut visited = HashSet::new();
    for line in lines {
        let p = split_string(line, " ");
        let times = p[1].to_i64();
        let dir = match p[0].as_str() {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => unreachable!(),
        };
        for i in 0..times {
            rope.step(Vec2::new(dir.0, dir.1));
            visited.insert(rope.tail);
            // println!(
            //     "-> ({}, {}) ({}, {})",
            //     rope.head.x, rope.head.y, rope.tail.x, rope.tail.y
            // );
        }
    }
    visited.len() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut knots = Vec::new();
    for i in 0..10 {
        knots.push(Vec2::new(0, 0));
    }
    let mut rope = BigRope { knots };
    let mut visited = HashSet::new();
    for line in lines {
        let p = split_string(line, " ");
        let times = p[1].to_i64();
        let dir = match p[0].as_str() {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => unreachable!(),
        };
        for i in 0..times {
            rope.step(Vec2::new(dir.0, dir.1));
            let tail = rope.knots.last().unwrap();
            visited.insert(tail.clone());
        }
    }
    visited.len() as i64
}
