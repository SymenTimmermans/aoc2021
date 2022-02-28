use colored::Colorize;

use aoc2021::read_strs;

fn read_map(file_path: &str) -> Vec<Vec<u8>> {
    let lines = read_strs(file_path);
    let mut grid: Vec<Vec<u8>> = vec![vec![9; lines[0].len()]; lines.len()];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c as u8 - b'0';
        }
    }
    grid
}

fn find_low_points(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut low_points: Vec<(usize, usize)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut neighbor_vals: Vec<u8> = Vec::new();
            if y > 0 {
                neighbor_vals.push(grid[y - 1][x]);
            }
            if y < grid.len() - 1 {
                neighbor_vals.push(grid[y + 1][x]);
            }
            if x > 0 {
                neighbor_vals.push(grid[y][x - 1]);
            }
            if x < grid[0].len() - 1 {
                neighbor_vals.push(grid[y][x + 1]);
            }
            let mut lower = neighbor_vals
                .iter()
                .filter(|&v| *v <= grid[y][x])
                .peekable();
            if lower.peek().is_none() {
                print!("{}", grid[y][x].to_string().black().on_white());
                low_points.push((y, x));
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
    low_points
}

fn get_values(positions: &[(usize, usize)], grid: &[Vec<u8>]) -> Vec<u8> {
    let mut values: Vec<u8> = vec![];
    for (y, x) in positions {
        values.push(grid[*y][*x]);
    }
    values
}

fn get_basin_size(pos: &(usize, usize), grid: &[Vec<u8>]) -> u32 {
    let mut visited: Vec<(usize, usize)> = vec![];

    let mut queue: Vec<(usize, usize)> = vec![*pos];
    while let Some((y, x)) = queue.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }
        if grid[y][x] == 9 {
            continue;
        }
        visited.push((y, x));
        if y > 0 {
            queue.push((y - 1, x));
        }
        if y < grid.len() - 1 {
            queue.push((y + 1, x));
        }
        if x > 0 {
            queue.push((y, x - 1));
        }
        if x < grid[0].len() - 1 {
            queue.push((y, x + 1));
        }
    }

    visited.len() as u32
}

fn day9() {
    let grid = read_map("input/day9.txt");
    let low_point_positions = find_low_points(&grid);
    let low_points = get_values(&low_point_positions, &grid);
    let risk_level_sum = low_points
        .iter()
        .fold(0u32, |sum, &val| sum + val as u32 + 1);

    println!("Day 9, part 1: {}", risk_level_sum);
}

fn day9b() {
    let grid = read_map("input/day9.txt");
    let low_point_positions = find_low_points(&grid);
    // each low point position is a basin. for each basin, find the size.
    let basin_sizes: Vec<u32> = low_point_positions
        .iter()
        .map(|p| get_basin_size(p, &grid))
        .collect();

    // get the three largest basins
    let mut sorted_basin_sizes = basin_sizes;
    sorted_basin_sizes.sort_unstable();
    let mut largest_basins: Vec<u32> = vec![];
    for i in 0..3 {
        largest_basins.push(sorted_basin_sizes[sorted_basin_sizes.len() - 1 - i]);
    }
    // print the three largest basins
    println!("Day 9, part 2: {:?}", largest_basins);
    println!("          mul: {}", largest_basins.iter().product::<u32>());
}

pub fn main() {
    day9();
    day9b();
}
