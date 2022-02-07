use std::iter::Sum;

use num::{Integer, Signed};

use super::read_single_line_csv;

fn align_cost<T>(positions: &[T], align: T) -> T
where
    T: Integer + Signed + Sum + Copy,
{
    positions.iter().map(|p| (align - *p).abs()).sum()
}

fn align_cost_b<T>(positions: &[T], align: T) -> T
where
    T: Integer + Signed + Sum + Copy,
{
    positions.iter().map(|p| { 
        let n = (align - *p).abs();
        // triangular number sequence
        // 1/2 n * (n + 1)
        // or (n * (n + 1)) / 2
        // or (n * (n + 1)) / (1 + 1)
        // uses T::one() to stay generic
        (n * (n + T::one())) / (T::one() + T::one())
    } ).sum()
}

pub fn day7() {
    let positions: Vec<i32> = read_single_line_csv("input/day7.txt");

    // get lowest value in vec
    let low = positions.iter().min().unwrap();
    let high = positions.iter().max().unwrap();

    let mut fuel = i32::MAX;
    let mut pos = 0;

    for n in *low..*high {
        let cost = align_cost(&positions, n);
        //println!("Align cost {} = {}", n, cost);
        if cost < fuel {
            fuel = cost;
            pos = n;
        }
    }

    println!("Best position: {}, costs {} fuel", pos, fuel);
}

pub fn day7b() {
    let positions: Vec<i32> = read_single_line_csv("input/day7.txt");

    // get lowest value in vec
    let low = positions.iter().min().unwrap();
    let high = positions.iter().max().unwrap();

    let mut fuel = i32::MAX;
    let mut pos = 0;

    for n in *low..*high {
        let cost = align_cost_b(&positions, n);
        //println!("Align cost {} = {}", n, cost);
        if cost < fuel {
            fuel = cost;
            pos = n;
        }
    }

    println!("Best position: {}, costs {} fuel", pos, fuel);
}