use std::io::BufRead;
use std::{fs::File, io::BufReader};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        // split on space
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts[0] == "forward" {
            Command::Forward(parts[1].parse().unwrap())
        } else if parts[0] == "down" {
            Command::Down(parts[1].parse().unwrap())
        } else {
            Command::Up(parts[1].parse().unwrap())
        }
    }
}

#[derive(Debug)]
struct Position {
    h: i32,
    d: i32,
    a: i32,
}

impl Position {
    fn new() -> Self {
        Position { h: 0, d: 0, a: 0 }
    }

    fn move_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(x) => self.h += x,
            Command::Down(x) => self.d += x,
            Command::Up(x) => self.d -= x,
        }
    }

    fn move_command2(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(x) => {
                self.h += x;
                self.d += self.a * x;
            }
            Command::Down(x) => self.a += x,
            Command::Up(x) => self.a -= x,
        }
    }

    fn distance(&self) -> i32 {
        self.h * self.d
    }
}

fn read_commands(file_path: &str) -> Vec<Command> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .map(|l| l.as_str().into())
        .collect()
}

fn day2() {
    let commands = read_commands("input/day2.txt");
    let mut pos = Position::new();
    for command in commands {
        pos.move_command(&command);
    }
    println!("{:?}, distance: {}", pos, pos.distance());
}

fn day2b() {
    let commands = read_commands("input/day2.txt");
    let mut pos = Position::new();
    for command in commands {
        pos.move_command2(&command);
    }
    println!("{:?}, distance: {}", pos, pos.distance());
}

fn main() {
    day2();
    day2b();
}