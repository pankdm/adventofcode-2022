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
    let path = format!("input/day11/{}", file);
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

pub fn remove_prefix(s: &str, prefix: &str) -> String {
    if let Some(index) = s.find(prefix) {
        return s.to_string().split_off(index + prefix.len());
    }
    s.to_string()
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op_value: i64,
    op: String,
    test_true: usize,
    test_false: usize,
    div: i64,
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut monkeys = Vec::new();

    let mut counter = 0;
    loop {
        let mut items = VecDeque::new();
        let mut div = -1;
        let mut test_true = 0;
        let mut test_false = 0;
        let mut op_value = -1;
        let mut op = "".to_string();
        if counter >= lines.len() {
            break;
        }
        for i in 0..7 {
            if counter >= lines.len() {
                break;
            }
            let line = &lines[counter];
            match i {
                0 => {}
                1 => {
                    let p = split_string(&remove_prefix(line, "Starting items: "), ", ");
                    for j in 0..p.len() {
                        items.push_back(p[j].to_i64());
                    }
                }
                2 => {
                    let ops = remove_prefix(line, "Operation: ");
                    if ops == "new = old * old" {
                        op = "sq".to_string();
                    } else {
                        let p = split_string(&ops, " ");
                        op = p[p.len() - 2].clone();
                        op_value = p.last().unwrap().to_i64();
                    }
                }
                3 => {
                    div = remove_prefix(line, "divisible by ").to_i64();
                }
                4 => {
                    test_true = remove_prefix(line, "throw to monkey ").to_i64() as usize;
                }
                5 => {
                    test_false = remove_prefix(line, "throw to monkey ").to_i64() as usize;
                }
                _ => {}
            }
            counter += 1;
        }
        // println!("{:?}", items);
        // println!("div = {}", div);
        monkeys.push(Monkey {
            items,
            test_false,
            test_true,
            div,
            op,
            op_value,
        });
    }
    let mut md = 1;
    for m in monkeys.iter() {
        md *= m.div;
    }

    let mut stats = vec![0; monkeys.len()];

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut m = monkeys[i].clone();
            loop {
                if let Some(item) = m.items.pop_front() {
                    stats[i] += 1;
                    let mut worry = item;
                    if m.op == "sq" {
                        // println!("worry = {}", worry);
                        worry *= worry;
                    } else {
                        if m.op == "+" {
                            worry += m.op_value;
                        } else {
                            assert!(m.op == "*");
                            worry *= m.op_value;
                        }
                    }
                    // worry = worry / 3;
                    worry = worry % md;
                    let mut dst;
                    if worry % m.div == 0 {
                        dst = m.test_true;
                    } else {
                        dst = m.test_false;
                    }
                    // println!(">> Monkey {} throwing {} to {}", i, worry, dst);
                    monkeys[dst].items.push_back(worry);
                } else {
                    break;
                }
            }
            monkeys[i] = m;
        }
        println!("round {}: {:?}", round, stats);
        // for i in 0..monkeys.len() {
        //     println!("  Monkey {} -> {:?}", i, monkeys[i].items);
        // }
    }
    stats.sort();
    stats.reverse();
    stats[0] * stats[1]
    // -1
}

// 0 -> [27, 27, 24, 20]
// 1 -> [2080, 401, 1046, 42850, 168, 26]
// 2 -> []
// 3 -> []

// Monkey 0: 20, 23, 27, 26
// Monkey 1: 2080, 25, 167, 207, 401, 1046
// Monkey 2:
// Monkey 3:

pub fn part2(lines: &Vec<String>) -> i64 {
    -1
}
