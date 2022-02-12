#![allow(dead_code)]

use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub use day1::*;
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