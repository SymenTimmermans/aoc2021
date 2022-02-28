use std::{collections::HashSet, str::FromStr};

use aoc2021::read_strs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    X(i32),
    Y(i32),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split on the = character
        let mut parts = s.split('=');
        let axis = parts.next().unwrap().chars().last().unwrap();
        // get the value, cast to i32
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        match axis {
            'x' => Ok(Fold::X(value)),
            'y' => Ok(Fold::Y(value)),
            _ => Err(()),
        }
    }
}

struct Paper {
    dots: HashSet<(i32, i32)>,
    folds: Vec<Fold>,
}

impl Paper {
    /// Read the paper from a file
    fn read_file(file_path: &str) -> Paper {
        // read filepath into a vec of strings
        let lines = read_strs(file_path);

        let dots = HashSet::from_iter(
            lines
                .iter()
                .filter(|l| !l.is_empty() && (*l).contains(','))
                .map(|l| {
                    let mut split = l.split(',');
                    let x = split.next().unwrap().trim().parse::<i32>().unwrap();
                    let y = split.next().unwrap().trim().parse::<i32>().unwrap();
                    (x, y)
                }),
        );

        let folds = lines
            .iter()
            .filter(|l| !l.is_empty() && (*l).contains("fold"))
            .map(|l| l.trim().parse::<Fold>().unwrap())
            .collect();

        Paper { dots, folds }
    }

    /// Execute folds a given number of times.
    fn fold(&mut self, times: usize) {
        // if times is bigger than the number of folds, panic!
        if times > self.folds.len() {
            panic!("Can't fold more than the number of folds!");
        }

        for i in 0..times {
            let fold = self.folds[i];
            self.do_fold(&fold);
        }

        // remove the number of items from the front of the vector
        self.folds.drain(0..times);
    }

    /// Execute the fold, updating the dots
    fn do_fold(&mut self, fold: &Fold) {
        // create a new set to hold the new dots
        let mut new_dots = HashSet::new();

        // loop through the dots and fold them
        for dot in &self.dots {
            let (mut x, mut y) = dot;

            // if the fold is on the X axis, fold the X axis
            match fold {
                Fold::X(fx) => {
                    if x > *fx {
                        // mirror x along fx axis
                        x = fx - (x - fx);
                    }
                }
                Fold::Y(fy) => {
                    if y > *fy {
                        // mirror y along fy axis
                        y = fy - (y - fy);
                    }
                }
            }

            new_dots.insert((x, y));
        }

        // replace the old dots with the new ones
        self.dots = new_dots;
    }

    /// The number of dots on the paper
    fn count_dots(&self) -> usize {
        // simply return the length of the dots hashset
        self.dots.len()
    }

    /// This function prints out a grid of dots, with dots marked with a #
    fn print_dots(&self) {
        // first we need to find the max x and y values
        let mut max_x = std::i32::MIN;
        let mut max_y = std::i32::MIN;

        for dot in &self.dots {
            let (x, y) = dot;
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        // create a char vec to hold the grid
        let mut grid = vec![' '; (max_x + 1) as usize * (max_y + 1) as usize];

        // loop through the dots and mark them with a #
        for dot in &self.dots {
            let (x, y) = dot;
            grid[(x + y * (max_x + 1)) as usize] = '#';
        }

        // print the grid
        for y in 0..=max_y {
            for x in 0..=max_x {
                print!("{}", grid[(x + y * (max_x + 1)) as usize]);
            }
            println!();
        }
    }
}

pub fn main() {
    // read the file
    let mut paper = Paper::read_file("input/day13.txt");

    // fold once
    paper.fold(1);

    // print the number of dots
    println!("Dots after 1 fold: {}", paper.count_dots());

    // fold until we have no more folds
    while !paper.folds.is_empty() {
        paper.fold(1);
    }

    // print the grid
    paper.print_dots();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let paper = Paper::read_file("input/day13_ex.txt");

        // count dots after reading
        assert_eq!(paper.count_dots(), 18);
    }

    #[test]
    fn test_fold_once() {
        let mut paper = Paper::read_file("input/day13_ex.txt");

        // fold the paper
        paper.fold(1);

        // count dots after folding
        assert_eq!(paper.count_dots(), 17);

        // one fold should remain
        assert_eq!(paper.folds.len(), 1);
    }

    #[test]
    fn test_fold_twice() {
        let mut paper = Paper::read_file("input/day13_ex.txt");

        // fold the paper
        paper.fold(2);

        // count dots after folding
        assert_eq!(paper.count_dots(), 16);

        // zero folds should remain
        assert_eq!(paper.folds.len(), 0);
    }
}
