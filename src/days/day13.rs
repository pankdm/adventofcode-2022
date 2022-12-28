// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::*;

pub fn read_main_input() -> String {
    let args = std::env::args().cv();
    let mut file = "in.txt".to_string();

    // Overwrite the input file, but not in test env
    #[cfg(not(test))]
    if args.len() >= 2 {
        file = args[1].to_string()
    }
    let path = format!("input/day13/{}", file);

    // read_input(&path)
    read_input_as_string(&path)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}

pub fn main() {
    let lines = read_main_input();

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Value(i64),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Value(i64),
    List(Vec<Packet>),
}

fn tokenize(a: &String) -> Vec<Token> {
    let mut res = Vec::new();
    let stream = a.cv();
    let mut index = 0;
    loop {
        if index >= stream.len() {
            break;
        }

        if !stream[index].is_numeric() {
            res.push(Token::Char(stream[index]));
            index += 1;
            continue;
        }
        let mut digits = Vec::new();
        while index + 1 < stream.len() && stream[index + 1].is_numeric() {
            digits.push(stream[index]);
            index += 1;
        }
        digits.push(stream[index]);
        index += 1;
        res.push(Token::Value(digits.to_str().to_i64()));
    }
    res
}

fn parse_impl(tokens: &Vec<Token>, offset: usize) -> (Packet, usize) {
    println!("parsing from offset {} at {:?}", offset, tokens[offset]);
    if let Token::Value(v) = tokens[offset] {
        return (Packet::Value(v), offset + 1);
    }
    if let Token::Char(ch) = tokens[offset] {
        assert_eq!(ch, '[');
        let mut res = Vec::new();
        let mut cur_offset = offset + 1;
        loop {
            if tokens[cur_offset] == Token::Char(']') {
                return (Packet::List(res), cur_offset + 1);
            } else if tokens[cur_offset] == Token::Char(',') {
                cur_offset += 1;
            }
            let (p, next_offset) = parse_impl(tokens, cur_offset);
            // println!(
            //     "at offset {}, got: {:?} next={}",
            //     cur_offset, p, next_offset
            // );
            match tokens[next_offset] {
                Token::Char(']') | Token::Char(',') => {}
                _ => unreachable!(),
            }
            res.push(p);
            cur_offset = next_offset;
        }
    }
    unreachable!();
}

fn parse(a: &String) -> Packet {
    let tokens = tokenize(a);
    // for i in 0..tokens.len() {
    //     println!("{} {:?}", i, tokens[i]);
    // }
    let (res, _) = parse_impl(&tokens, 0);
    res
}

fn compare(a: &Packet, b: &Packet) -> i64 {
    // println!("comparing {:?} vs {:?}", a, b);
    if let Packet::Value(va) = a {
        if let Packet::Value(vb) = b {
            if va == vb {
                return 0;
            }
            if va < vb {
                return -1;
            }
            if va > vb {
                return 1;
            }
        }
        let lista = Packet::List(vec![a.clone()]);
        return compare(&lista, b);
    }
    if let Packet::List(vec_a) = a {
        if let Packet::Value(vb) = b {
            let listb = Packet::List(vec![b.clone()]);
            return compare(a, &listb);
        }

        if let Packet::List(vec_b) = b {
            // println!("both are lists, comparing");
            let mut index_a = 0;
            let mut index_b = 0;
            let res = loop {
                if index_a >= vec_a.len() && index_b >= vec_b.len() {
                    break 0;
                }
                if index_a >= vec_a.len() {
                    break -1;
                }
                if index_b >= vec_b.len() {
                    break 1;
                }
                let cmp = compare(&vec_a[index_a], &vec_b[index_b]);
                if cmp != 0 {
                    break cmp;
                }
                index_a += 1;
                index_b += 1;
            };
            println!("comparing {:?} vs {:?} -> res = {}", a, b, res);
            return res;
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }
}

pub fn part1(data: &String) -> i64 {
    let pairs = split_string(data, "\n\n");
    let mut res = 0;
    for (index, pair) in pairs.iter().enumerate() {
        let lines = split_string(&pair, "\n");
        let a = parse(&lines[0]);
        let b = parse(&lines[1]);
        // println!("1: {}, {:?}", lines[0], a);
        // println!("2: {}, {:?}", lines[1], b);
        let cmp = compare(&a, &b);
        // println!("{}, cmp = {}\n", index + 1, cmp);
        if cmp == -1 {
            res += (index + 1) as i64;
        }
    }
    res
}

pub fn part2(data: &String) -> i64 {
    let pairs = split_string(data, "\n\n");

    let mut all = Vec::new();
    let mut all_lines = Vec::new();

    for (index, pair) in pairs.iter().enumerate() {
        let lines = split_string(&pair, "\n");
        all_lines.push(lines[0].clone());
        all_lines.push(lines[1].clone());
    }
    all_lines.push("[[2]]".to_string());
    all_lines.push("[[6]]".to_string());

    for line in all_lines.iter() {
        let p = parse(line);
        all.push((p, line.clone()));
    }

    all.sort_by(|x, y| match compare(&x.0, &y.0) {
        -1 => Ordering::Less,
        0 => Ordering::Equal,
        1 => Ordering::Greater,
        _ => unreachable!(),
    });

    let mut res = 1;
    for (index, value) in all.iter().enumerate() {
        if value.1 == "[[2]]" {
            res *= (index + 1) as i64;
        }
        if value.1 == "[[6]]" {
            res *= (index + 1) as i64;
        }
    }

    res
}
