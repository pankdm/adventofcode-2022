// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

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
    let path = format!("input/day22/{}", file);
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

#[derive(Default)]
struct Solver {
    grid: Vec<Vec<char>>,
    // debug_grid: Vec<Vec<char>>,
    x_bounds: Vec<(i64, i64)>,
    y_bounds: Vec<(i64, i64)>,
    x_max: usize,
    y_max: usize,
    is_part2: bool,
    cube_mapping: HashMap<(Vec2, Vec2), (Vec2, Vec2)>,
}

fn convert_facing(dir: Vec2) -> i64 {
    return match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        (_, _) => unreachable!(),
    };
}

fn show_facing(dir: Vec2) -> char {
    return match (dir.x, dir.y) {
        (1, 0) => '>',
        (0, 1) => 'v',
        (-1, 0) => '<',
        (0, -1) => '^',
        (_, _) => unreachable!(),
    };
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.to_str());
    }
}

impl Solver {
    pub fn init(&mut self, lines: &Vec<String>, is_part2: bool) {
        self.is_part2 = is_part2;

        let mut grid = Vec::new();
        for line in lines.iter() {
            let row = line.to_vec();
            grid.push(row);
        }

        let row_max = grid.len();
        let col_max = grid.iter().map(|x| x.len()).max().unwrap();

        for row in &mut grid {
            while row.len() < col_max {
                row.push(' ');
            }
        }

        let mut row_bounds = Vec::new();
        let mut col_bounds = Vec::new();

        for row_idx in 0..grid.len() {
            let chars = || (0..grid[row_idx].len()).map(|i| grid[row_idx][i]);
            let start_idx = chars().position(|c| c != ' ').unwrap() as i64;
            let end_idx = chars().rposition(|c| c != ' ').unwrap() as i64;
            row_bounds.push((start_idx, end_idx));
            println!("row = {}, bounds {:?}", row_idx, (start_idx, end_idx));
        }

        for col_idx in 0..col_max {
            let chars = || (0..grid.len()).map(|i| grid[i][col_idx]);
            let start_idx = chars().position(|c| c != ' ').unwrap() as i64;
            let end_idx = chars().rposition(|c| c != ' ').unwrap() as i64;
            col_bounds.push((start_idx, end_idx));
            println!("col = {}, bounds {:?}", col_idx, (start_idx, end_idx));
        }

        // self.debug_grid = grid.clone();
        self.grid = grid;
        self.x_bounds = row_bounds;
        self.y_bounds = col_bounds;
        self.x_max = col_max;
        self.y_max = row_max;

        self.create_mapping();
    }

    fn add_mapping(&mut self, src: Vec2, src_dir: Vec2, dst: Vec2, dst_dir: Vec2) {
        self.add_mapping_helper(src, src_dir, dst, dst_dir);
        self.add_mapping_helper(dst, dst_dir * -1, src, src_dir * -1);
    }

    fn add_mapping_helper(&mut self, src: Vec2, src_dir: Vec2, dst: Vec2, dst_dir: Vec2) {
        // check that step actually outside of boundaries
        let next_src = src + src_dir;
        if self.grid.inside(next_src) && self.grid.get(next_src) != ' ' {
            println!(
                "Expected empty field, got: {} at {:?}, dir = {:?}",
                self.grid.get(next_src),
                src,
                src_dir
            );
            assert!(false);
        }
        self.cube_mapping.insert((src, src_dir), (dst, dst_dir));
    }

    fn create_mapping(&mut self) {
        let size = (self.grid.len() / 4) as i64;
        println!("using size = {}", size);

        //  12
        //  3
        // 45
        // 6
        let left = Vec2::new(-1, 0);
        let right = Vec2::new(1, 0);
        let up = Vec2::new(0, -1);
        let down = Vec2::new(0, 1);

        if self.is_part2 {
            // hardcoded for input
            // 1 left <-> 4 left
            {
                let x = size;
                let new_x = 0;
                for y in 0..size {
                    let new_y = 2 * size + (size - 1 - y);
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, left, dst, right);
                }
            }
            // 1 up <-> 6 left
            {
                let y = 0;
                let new_x = 0;
                for x in size..2 * size {
                    let new_y = 3 * size + (x - size);
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, up, dst, right);
                }
            }
            // 2 up <-> 6 down
            {
                let y = 0;
                let new_y = 3 * size + size - 1;
                for x in 2 * size..3 * size {
                    let new_x = x - 2 * size;
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, up, dst, up);
                }
            }
            // 2 right <-> 5 left
            {
                let x = 2 * size + size - 1;
                let new_x = size + (size - 1);
                for y in 0..size {
                    let new_y = 2 * size + (size - 1 - y);
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, right, dst, left);
                }
            }
            // 3 left <-> 4 up
            {
                let x = size;
                let new_y = 2 * size;
                for y in size..2 * size {
                    let new_x = y - size;
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, left, dst, down);
                }
            }
            // 3 right <-> 2 down
            {
                let x = size + size - 1;
                let new_y = size - 1;
                for y in size..2 * size {
                    let new_x = 2 * size + (y - size);
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, right, dst, up);
                }
            }
            // 5 down <-> 6 right
            {
                let y = 2 * size + size - 1;
                let new_x = size - 1;
                for x in size..2 * size {
                    let new_y = 3 * size + (x - size);
                    let src = Vec2::new(x, y);
                    let dst = Vec2::new(new_x, new_y);
                    self.add_mapping(src, down, dst, left);
                }
            }
        }
        assert_eq!(self.cube_mapping.len() % (size as usize), 0);
        println!(
            "total sides covered = {}",
            self.cube_mapping.len() / (size as usize)
        );
        // check that mapping is at non-empty cells
        for (from, to) in self.cube_mapping.iter() {
            let (src, _) = from;
            let (dst, _) = to;
            assert_ne!(self.grid.get(*src), ' ');
            assert_ne!(self.grid.get(*dst), ' ');
        }
    }

    fn solve(&self, path: &String) -> i64 {
        let pos = Vec2::new(50, 0);
        let (ok, final_pos, dir) = self.follow_path(pos, path);
        if ok {
            println!(
                "FOUND SOLUTION! row = {}, col = {}, facing = {}",
                final_pos.x + 1,
                final_pos.y + 1,
                convert_facing(dir)
            );
            return (final_pos.y + 1) * 1000 + (final_pos.x + 1) * 4 + convert_facing(dir);
        }
        -1
    }

    // returns cell and new facing direction
    fn compute_next_cell(&self, now: Vec2, dir: Vec2) -> (Vec2, Vec2) {
        if self.is_part2 {
            return self.compute_wrapping(now, dir);
        }

        let mut now = now + dir;
        if dir.x == 0 {
            let (y_min, y_max) = self.y_bounds[now.x as usize];
            if now.y > y_max {
                now.y = y_min
            } else if now.y < y_min {
                now.y = y_max;
            }
        } else {
            assert_eq!(dir.y, 0);
            let (x_min, x_max) = self.x_bounds[now.y as usize];
            if now.x > x_max {
                now.x = x_min
            } else if now.x < x_min {
                now.x = x_max;
            }
        }
        return (now, dir);
    }
    fn is_inside(&self, now: Vec2, dir: Vec2) -> bool {
        let next = now + dir;
        if dir.x == 0 {
            let (y_min, y_max) = self.y_bounds[now.x as usize];
            return y_min <= next.y && next.y <= y_max;
            // assert!(next.y <= y_max);
            // assert!(next.y >= y_min);
        } else {
            let (x_min, x_max) = self.x_bounds[now.y as usize];
            return x_min <= next.x && next.x <= x_max;
        }
    }

    fn compute_wrapping(&self, now: Vec2, dir: Vec2) -> (Vec2, Vec2) {
        let key = (now, dir);
        if self.cube_mapping.contains_key(&key) {
            if self.is_inside(now, dir) == true {
                println!("Expected outside, got inside at {:?}, {:?}", now, dir);
                println!("  mapping = {:?}", self.cube_mapping[&key]);
                assert!(false);
            }
            return self.cube_mapping[&key];
        }
        assert_eq!(self.is_inside(now, dir), true);
        let next = now + dir;
        (next, dir)
    }

    fn follow_path(&self, start: Vec2, path: &String) -> (bool, Vec2, Vec2) {
        let mut debug_grid = self.grid.clone();

        println!("following path at {:?}", start);
        assert_eq!(self.grid.get(start), '.');
        let grid: &Vec<Vec<char>> = &self.grid;
        let re = Regex::new(r"(\d+|[RL])").unwrap();
        let parts = re.find_iter(&path).map(|m| m.as_str().to_owned()).cv();
        // println!("{:?}", parts);

        let mut now = start;
        let mut dir = Vec2::new(1, 0);
        for part in parts.iter() {
            debug_grid.set(now, show_facing(dir));
            let cur_cell = self.grid.get(now);
            assert_eq!(cur_cell, '.');
            match part.as_str() {
                "L" => dir = dir.rotate_right(),
                "R" => dir = dir.rotate_left(),
                _ => {
                    let steps = part.to_i64();
                    for i in 0..steps {
                        // println!(" at {:?}, dir={:?}, stepping {}", now, dir, part);
                        debug_grid.set(now, show_facing(dir));
                        let (next_pos, next_facing) = self.compute_next_cell(now, dir);
                        let cell = self.grid.get(next_pos);
                        assert_ne!(cell, ' ');
                        if cell == '#' {
                            // don't move if there is a wall
                        } else {
                            now = next_pos;
                            dir = next_facing;
                        }
                    }
                }
            }
            // print_grid(&debug_grid);
        }
        (true, now, dir)
    }
}
pub fn part1(lines: &Vec<String>) -> i64 {
    let mut lines = lines.clone();
    let path = lines.pop().unwrap();
    lines.pop();
    println!("{}", path.len());

    let mut solver = Solver::default();
    solver.init(&lines, false);
    let res = solver.solve(&path);

    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut lines = lines.clone();
    let path = lines.pop().unwrap();
    lines.pop();
    println!("{}", path.len());

    let mut solver = Solver::default();
    solver.init(&lines, true);
    let res = solver.solve(&path);

    res
}
