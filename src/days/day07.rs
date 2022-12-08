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
    let path = format!("input/day07/{}", file);
    read_input(&path)
    // read_input_as_string(&path)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}

pub fn main() {
    let lines = read_main_input();

    let (p1, p2) = both_parts(&lines);
    println!("part1 = {}", p1);
    println!("part2 = {}", p2);
}

struct Node {
    id: usize,
    parent: i32,
    childs: HashMap<String, usize>,
    files: HashMap<String, i64>,
}

struct FileSystem {
    nodes: Vec<Node>,
    current: usize,
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut nodes = Vec::new();
        let current = 0;
        let root = Node {
            id: 0,
            parent: -1,
            childs: HashMap::new(),
            files: HashMap::new(),
        };
        nodes.push(root);
        FileSystem { nodes, current }
    }

    fn cd(&mut self, dir: String) {
        // println!("cd {} at current {}", dir, self.current);
        let node_count = self.nodes.len();
        let cur = &mut self.nodes[self.current];
        if dir == ".." {
            assert_ne!(cur.parent, -1);
            self.current = cur.parent as usize;
            return;
        }

        if !cur.childs.contains_key(&dir) {
            let node = Node {
                id: node_count,
                parent: self.current as i32,
                childs: HashMap::new(),
                files: HashMap::new(),
            };
            // println!(" . created node {}, id={}", dir, node.id);
            cur.childs.insert(dir, node.id);
            self.current = node.id;
            self.nodes.push(node);
        } else {
            self.current = cur.childs[&dir];
        }
    }

    fn file(&mut self, name: String, size: i64) {
        // println!("added file {}", name);
        self.nodes[self.current].files.insert(name, size);
    }

    // fn get_dir_sizes(&self) -> Vec<(String, i64)> {
    //     let mut res = Vec::new();
    //     total_size = self.recursive_get(0, &mut res);
    //     res,
    // }

    // returns size of current dir and populates result
    fn recursive_get(&self, current: usize, res: &mut Vec<(String, i64)>) -> i64 {
        let cur = &self.nodes[current];
        let mut total_size = cur.files.iter().map(|(_, v)| v).sum();
        for (k, v) in cur.childs.iter() {
            let size = self.recursive_get(*v, res);
            res.push((k.clone(), size));
            total_size += size;
        }
        // println!("recursive_get at cur={}, res = {:?}", current, res);
        total_size
    }
}

pub fn both_parts(lines: &Vec<String>) -> (i64, i64) {
    let mut fs = FileSystem::new();

    for line in lines {
        let p = split_string(line, " ");
        if p[0] == "$" {
            if p[1] == "cd" {
                fs.cd(p[2].clone());
            }
        } else {
            if p[0] == "dir" {
            } else {
                fs.file(p[1].clone(), p[0].to_i64());
            }
        }
    }
    let mut res = Vec::new();
    let total_size = fs.recursive_get(0, &mut res);
    // println!("{:#?}", res);
    let ans_part1 = res.iter().map(|(k, v)| v).filter(|x| **x <= 100000).sum();
    let unused_space = 70000000 - total_size;
    let ans_part2 = res
        .iter()
        .map(|(k, v)| v)
        .filter(|v| unused_space + **v >= 30000000)
        .min()
        .unwrap();
    (ans_part1, *ans_part2)
}
