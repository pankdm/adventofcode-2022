// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::time::Instant;

// Some basic includes to always include
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

extern crate aoc;
use aoc::*;
use days::*;

fn main() {
    let now = Instant::now();

    day12::main();

    let elapsed_ms = now.elapsed().as_millis();
    println!("Finished in {}ms", elapsed_ms);
}
