#![allow(dead_code)]

use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

pub fn read_ints(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .map(|l| l.parse::<i32>().expect("failed to parse int"))
        .collect()
}

pub fn read_strs(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .collect()
}

pub fn read_single_line_csv<T>(path: &str) -> Vec<T>
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
