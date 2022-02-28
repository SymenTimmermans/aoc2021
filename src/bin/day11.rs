use colored::Colorize;

use aoc2021::read_strs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Octopus {
    Idle(usize),
    Flashing,
    HasFlashed,
}

impl Octopus {
    fn increase(&mut self) {
        match self {
            Octopus::Idle(i) => *i += 1,
            Octopus::Flashing => {}
            Octopus::HasFlashed => {}
        }
    }

    fn should_flash(&self) -> bool {
        matches!(self, Octopus::Idle(e) if *e > 9)
    }

    fn flash_if_should(&mut self) -> u32 {
        if self.should_flash() {
            *self = Octopus::Flashing;
            return 1;
        }
        0
    }

    fn reset_if_flashed(&mut self) {
        if matches!(self, Octopus::HasFlashed) {
            *self = Octopus::Idle(0);
        }
    }
}

fn read_octopi(file_path: &str) -> Vec<Vec<Octopus>> {
    let lines = read_strs(file_path);
    let mut octopi = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Octopus::Idle(c as usize - '0' as usize));
        }
        octopi.push(row);
    }
    octopi
}

fn print_octopi(octopi: &[Vec<Octopus>]) {
    for row in octopi {
        for octopus in row {
            match octopus {
                Octopus::Idle(0) => print!("{}", String::from("0").yellow()),
                Octopus::Idle(e) => print!("{}", e.to_string().blue()),
                Octopus::Flashing => print!("{}", String::from("*").black().on_white()),
                Octopus::HasFlashed => print!("{}", String::from("0").black().on_white()),
            }
        }
        println!();
    }
}

fn has_octopi_that_should_flash(octopi: &[Vec<Octopus>]) -> bool {
    for row in octopi {
        for octopus in row {
            if octopus.should_flash() {
                return true;
            }
        }
    }
    false
}

/// This function processes the octopi in the given octopi matrix.
/// First, increase the energy of all the octopuses by one.
/// Then, search for "flashing" octopi.
/// An octopus with energy level greater than 9 will become "flashing"
/// For every "flashing" octopus, increase it's neighbours energy by one.
/// If any of the neighbors increase energy above 9, they become "flashing" too
/// and spread their energy to their neighbours.
/// This continues until no more octopi have energy levels above 9.
/// The last step, every flashing octopus is set to zero.
fn step(octopi: &mut [Vec<Octopus>]) -> u32 {
    // increase all energy by one
    for row in 0..octopi.len() {
        for col in 0..octopi[0].len() {
            octopi[row][col].increase();
        }
    }

    // as long as there are octopi with energy levels above 9
    // do the flashing step

    // keep count of the flashes
    let mut flashes: u32 = 0;

    while has_octopi_that_should_flash(octopi) {
        for row in 0..octopi.len() {
            for col in 0..octopi[0].len() {
                flashes += octopi[row][col].flash_if_should();
            }
        }

        // increase neighbours of flashing octopi energy by one
        for row in 0..octopi.len() {
            for col in 0..octopi[0].len() {
                let octopus = &octopi[row][col];
                if let Octopus::Flashing = octopus {
                    if row > 0 {
                        octopi[row - 1][col].increase();
                        if col > 0 {
                            octopi[row - 1][col - 1].increase();
                        }
                        if col < octopi[0].len() - 1 {
                            octopi[row - 1][col + 1].increase();
                        }
                    }
                    if row < octopi.len() - 1 {
                        octopi[row + 1][col].increase();
                        if col > 0 {
                            octopi[row + 1][col - 1].increase();
                        }
                        if col < octopi[0].len() - 1 {
                            octopi[row + 1][col + 1].increase();
                        }
                    }
                    if col > 0 {
                        octopi[row][col - 1].increase();
                    }
                    if col < octopi[0].len() - 1 {
                        octopi[row][col + 1].increase();
                    }
                    // Mark the octopus as "has flashed"
                    octopi[row][col] = Octopus::HasFlashed;
                }
            }
        }
    }

    // set all flashing octopi to zero
    for row in 0..octopi.len() {
        for col in 0..octopi[0].len() {
            octopi[row][col].reset_if_flashed();
        }
    }

    flashes
}

fn day11() {
    let mut octopi = read_octopi("input/day11.txt");
    let mut total_flashes = 0;
    println!("Before any steps:");
    print_octopi(&octopi);

    for i in 1..101 {
        let flashes = step(&mut octopi);
        total_flashes += flashes;

        if i % 10 == 0 {
            println!("\nAfter step {}:", i);
            print_octopi(&octopi);
            println!("Total flashes: {}", total_flashes);
        }
    }
}

fn day11b() {
    let mut octopi = read_octopi("input/day11.txt");

    // get the total number of octopi
    let total_octopi: u32 = octopi.iter().map(|row| row.len()).sum::<usize>() as u32;

    println!("Total nr of octopi: {}", total_octopi);

    for i in 1..1001 {
        let flashes = step(&mut octopi);
        print!(".");

        // if the number of flashes equals the number of octopi, they all flashed :-)
        if flashes == total_octopi {
            println!("\nAfter step {}:", i);
            print_octopi(&octopi);
            break;
        }
    }
}

pub fn main() {
    day11();
    day11b();
}
