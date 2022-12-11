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
    let path = format!("input/day10/{}", file);
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

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut values = HashMap::new();
    let mut counter = 1;
    let mut x = 1;
    for line in lines {
        values.insert(counter, x);
        if line == "noop" {
            counter += 1;
        } else {
            let p = split_string(line, " ");
            let delta = p[1].to_i64();
            values.insert(counter + 1, x);
            counter += 2;
            x += delta;
        }
    }
    let mut res = 0;
    for i in (20..=220).step_by(40) {
        res += i * values[&i];
    }

    let mut screen = vec![' '; 240];
    for i in 0..240 {
        let pos = values[&(i + 1)];
        if (pos - (i % 40)).abs() <= 1 {
            screen[i as usize] = '#';
        }
    }
    for i in 0..6 {
        for j in 0..40 {
            print!("{}", screen[j + i * 40]);
        }
        println!("")
    }

    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    -1
}
