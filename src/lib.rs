#![feature(portable_simd)]

#[macro_use]
mod macros;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2024 }

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
