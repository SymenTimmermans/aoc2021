#![allow(dead_code)]

use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub use day2::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;
pub use day7::*;

fn read_ints(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .map(|l| l.parse::<i32>().expect("failed to parse int"))
        .collect()
}

fn read_strs(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .collect()
}

fn read_single_line_csv<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let lines = read_strs(path);
    lines[0]
        .split(',')
        .map(str::parse::<T>)
        .map(Result::unwrap)
        .collect()
}

pub fn day1() {
    let depths = read_ints("input/day1.txt");
    let mut prev = -1;
    let mut inc_count = 0;
    for depth in depths {
        if prev == -1 {
            println!("{} n/a", depth);
        } else if prev > depth {
            println!("{} decreased", depth);
        } else {
            println!("{} increased", depth);
            inc_count += 1;
        }
        prev = depth;
    }
    println!("inc_count: {}", inc_count);
}

pub fn day1b() {
    let depths = read_ints("input/day1.txt");
    let mut last_win = -1;
    let mut inc_count = 0;
    for i in 0..depths.len() - 2 {
        let win = &depths[i..i + 3].iter().sum::<i32>();
        if last_win == -1 {
            println!("{} n/a", win);
        } else if last_win == *win {
            println!("{} no change", win);
        } else if last_win > *win {
            println!("{} decreased", win);
        } else {
            println!("{} increased", win);
            inc_count += 1;
        }
        last_win = *win;
    }
    println!("inc_count: {}", inc_count);
}
