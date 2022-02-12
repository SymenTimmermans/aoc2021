use crate::days::read_ints;

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
