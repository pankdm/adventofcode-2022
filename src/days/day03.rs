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
    let path = format!("input/day03/{}", file);
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

pub fn priority(c: char) -> i64 {
    if 'a' <= c && c <= 'z' {
        return c as i64 - 'a' as i64 + 1;
    }
    return c as i64 - 'A' as i64 + 27;
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for line0 in lines {
        let line = line0.to_vec();
        assert!(line.len() % 2 == 0);
        let n = line.len() / 2;
        let mut found = false;
        for i in 0..n {
            if found {
                break;
            }
            for j in n..line.len() {
                if line[i] == line[j] {
                    res += priority(line[i]);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    res
}

pub fn find_priority(counter: usize, lines: &Vec<String>) -> i64 {
    for c1 in lines[counter].to_vec() {
        for c2 in lines[counter + 1].to_vec() {
            for c3 in lines[counter + 2].to_vec() {
                if c1 == c2 && c2 == c3 {
                    return priority(c1);
                }
            }
        }
    }
    unreachable!();
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut counter = 0;
    loop {
        if counter >= lines.len() {
            break;
        }
        res += find_priority(counter, lines);
        counter += 3;
    }
    res
}
