use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl From<&str> for Position {
    fn from(input: &str) -> Self {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        Position { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Position,
    end: Position,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let mut parts = s.split(" -> ");
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();
        Line {
            start: start.into(),
            end: end.into(),
        }
    }
}

impl Line {
    fn get_points(&self) -> Vec<Position> {
        // return all the points between start and end, inclusive
        let mut points = Vec::new();
        let mut x = self.start.x;
        let mut y = self.start.y;
        let xd = (self.end.x - self.start.x).signum();
        let yd = (self.end.y - self.start.y).signum();
        while x != self.end.x || y != self.end.y {
            points.push(Position { x, y });
            x += xd;
            y += yd;
        }
        // add the last point
        points.push(self.end);
        points
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

fn read_lines(filename: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.unwrap().as_str().into());
    }
    lines
}

fn day5() {
    let lines = read_lines("input/day5.txt");

    let mut vent_map: HashMap<Position, i32> = HashMap::new();

    for line in lines {
        if line.is_diagonal() {
            println!("{:?} is diagonal, skip", line);
            continue;
        }

        let points_in_line = line.get_points();
        for point in points_in_line {
            println!("{:?} covers point {:?}", line, point);

            // mark this point on the vent_map, increase if already present
            let count = vent_map.entry(point).or_insert(0);
            *count += 1;
        }
    }

    // now count the number of points that are covered by more than one line
    let mut count = 0;
    for (_, v) in vent_map {
        if v > 1 {
            count += 1;
        }
    }

    println!("{} points are covered by more than one line", count);
}

fn day5b() {
    let lines = read_lines("input/day5.txt");

    let mut vent_map: HashMap<Position, i32> = HashMap::new();

    for line in lines {
        let points_in_line = line.get_points();
        for point in points_in_line {
            // mark this point on the vent_map, increase if already present
            let count = vent_map.entry(point).or_insert(0);
            *count += 1;
        }
    }

    // now count the number of points that are covered by more than one line
    let mut count = 0;
    for (_, v) in vent_map {
        if v > 1 {
            count += 1;
        }
    }

    println!("{} points are covered by more than one line", count);
}

pub fn main() {
    day5();
    day5b();
}