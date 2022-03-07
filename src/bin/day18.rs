use std::{fmt::Display, ops::Add, str::FromStr};

use aoc2021::read_strs;
use itertools::Itertools;

/// Snailfish numbers
/// -----------------
/// Snailfish numbers aren't like regular numbers. Instead, every snailfish number
/// is a pair - an ordered list of two elements. Each element of the pair can be
/// either a regular number or another pair.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Number(i32),
    Pair(Box<Snailfish>),
}

/// This allows us to read an element from a string.
impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if the string contains a comma, its a pair
        if s.contains(',') {
            // which means we can parse the string as a Snailfish number
            // and return a Pair enum containing the result
            Ok(Element::Pair(Box::new(s.parse::<Snailfish>()?)))
        } else {
            // otherwise, we can parse the string as a regular number
            // and return a Number enum containing the result
            Ok(Element::Number(s.parse::<i32>().expect("Not a number")))
        }
    }
}

/// Implement display so it's easier to print and test.
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Pair(p) => write!(f, "{}", p),
        }
    }
}

impl Element {
    /// Returns the leftmost number in the entire (sub-)tree.
    fn leftmost_number(&mut self) -> Option<&mut Self> {
        match self {
            Element::Number(_) => Some(self),
            Element::Pair(p) => p.leftmost_number(),
        }
    }

    /// Returns the rightmost number in the entire (sub-)tree.
    fn rightmost_number(&mut self) -> Option<&mut Self> {
        match self {
            Element::Number(_) => Some(self),
            Element::Pair(p) => p.rightmost_number(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snailfish {
    left: Element,
    right: Element,
}

/// We want to be able to read in a snailfish number from a string.
impl FromStr for Snailfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ignore the first and last character
        let s = &s[1..s.len() - 1];
        // walk through the string, counting opening and closing brackets
        let mut open = 0;
        let mut close = 0;
        let mut commapos = None;
        // also keep a string that holds the current element we're looking at
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => open += 1,
                ']' => close += 1,
                ',' => {
                    // if we find a comma and we have equal numbers of opening and
                    // closing braces, we have found the position of the comma to split on
                    if open == close {
                        commapos = Some(i);
                    }
                }
                _ => (),
            }
        }
        // if we found a comma, split the string at the position of the comma
        if let Some(commapos) = commapos {
            let (left, right) = s.split_at(commapos);
            // take the first character off the right string, it is the comma
            let right = &right[1..];
            // parse the left and right parts
            let left = left.parse::<Element>()?;
            let right = right.parse::<Element>()?;
            // return the pair
            Ok(Snailfish { left, right })
        } else {
            // if we didn't find a comma, errr
            Err(())
        }
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

/// we want to be able to add two snailfish numbers
/// so lets get cheeky and implement the add operator
impl Add for Snailfish {
    type Output = Snailfish;

    fn add(self, other: Self) -> Self::Output {
        let mut sf = Snailfish {
            left: Element::Pair(Box::new(self)),
            right: Element::Pair(Box::new(other)),
        };
        // every add triggers a reduce
        sf.reduce();
        sf
    }
}

impl Snailfish {
    /// return true if the number needs reducing
    fn needs_reducing(&self) -> bool {
        // if any pair is nested inside four pairs, we need to reduce (explode)
        // or if any number is 10 or higher, we need to reduce (split)
        self.needs_exploding(1) || self.needs_splitting()
    }

    /// check if pair is nested, can be called recursively to find nesting levels
    /// of over 4 pairs
    fn needs_exploding(&self, level: usize) -> bool {
        // if any pair is nested inside four pairs, we need to reduce
        // if level is 4 and either left or right is a pair, return true
        if level == 4 {
            if let Element::Pair(_) = self.left {
                return true;
            }
            if let Element::Pair(_) = self.right {
                return true;
            }
            false
        } else {
            // if either left or right is a pair, execute the function recursively
            // on the pair
            let left_nested = if let Element::Pair(p) = &self.left {
                p.needs_exploding(level + 1)
            } else {
                false
            };
            let right_nested = if let Element::Pair(p) = &self.right {
                p.needs_exploding(level + 1)
            } else {
                false
            };
            // return true if either left or right is a nested pair
            left_nested || right_nested
        }
    }

    /// check if number has any nested numbers 10 or higher
    fn needs_splitting(&self) -> bool {
        // if any number is 10 or higher, we need to split
        // if either left or right is a number and it is 10 or higher, return true
        if let Element::Number(n) = &self.left {
            if *n >= 10 {
                return true;
            }
        }
        if let Element::Number(n) = &self.right {
            if *n >= 10 {
                return true;
            }
        }
        let left_needs_splitting = if let Element::Pair(p) = &self.left {
            p.needs_splitting()
        } else {
            false
        };
        let right_needs_splitting = if let Element::Pair(p) = &self.right {
            p.needs_splitting()
        } else {
            false
        };
        left_needs_splitting || right_needs_splitting
    }

    /// reduce the number
    fn reduce(&mut self) {
        while self.needs_reducing() {
            // if we need to explode, explode
            if self.needs_exploding(1) {
                self.explode(1, (None, None));
            } else {
                // if we need to split, split
                if self.needs_splitting() {
                    self.split();
                }
            }
        }
    }

    /// Explode the number. If the number at a certain level needs exploding,
    /// it will bubble up its contents, and these two numbers will be added to
    /// the numbers to the left or right of the pair.
    /// We need to return a boolean to indicate if there was an explosion, so we don't
    /// explode two times.
    fn explode(
        &mut self,
        level: usize,
        neighbours: (Option<&mut Element>, Option<&mut Element>),
    ) -> bool {
        // The trouble with this algorithm is that it's not only recursive, but
        // it also bubbles up in two directions, and should bubble down the exploded
        // pair again.
        // There's a few assumptions we can make here:
        // 1. The Snailfish number has a pair that should explode.
        // 2. We only explode 1 number at a time.
        // 3. There's optionally one number to the left and optionally one to the right.
        // So, if we can traverse the tree, and just keep track of references to those
        // three elements that we need to change, we only need to do one manipulation.

        // keep track of what we exploded so we can replace the arm outside the match
        // scope.
        let mut left_exploded = false;
        let mut right_exploded = false;

        //dbg!(&self.to_string());
        if level == 4 {
            // if we have a pair here, do the explosion and addition to the neighbours
            // and return, because we are done.
            let mut add_left = 0;
            let mut add_right = 0;

            // if the left is a pair
            if let Element::Pair(lp) = &self.left {
                // if the left pair has a number on the left
                if let Element::Number(lpl) = lp.left {
                    add_left = lpl;
                }
                // the right term of the pair should be a number
                if let Element::Number(lpr) = lp.right {
                    add_right = lpr;
                }

                // if the left is a pair, the right is either a number or a pair,
                // so we should get the reference to the leftmost number of that
                // branch.
                let neighbour_right = self.right.leftmost_number();

                // add the left, if the left neighbour is a number element
                if let Some(n) = neighbours.0 {
                    if let Element::Number(nl) = n {
                        *n = Element::Number(*nl + add_left);
                    }
                }

                // add the right, if the right neighbour is a number element
                if let Some(n) = neighbour_right {
                    if let Element::Number(nr) = n {
                        *n = Element::Number(*nr + add_right);
                    }
                }

                left_exploded = true;
            } else if let Element::Pair(rp) = &self.right {
                // if the right is a pair
                if let Element::Number(rpl) = rp.left {
                    add_left = rpl;
                }
                // the left term of the pair should be a number
                if let Element::Number(rpr) = rp.right {
                    add_right = rpr;
                }

                // if the right is a pair, the left is either a number or a pair,
                // so we should get the reference to the rightmost number of that
                // branch.
                let neighbour_right = self.left.rightmost_number();

                // add the left, if the left neighbour is a number element
                if let Some(n) = neighbour_right {
                    if let Element::Number(nl) = n {
                        *n = Element::Number(*nl + add_left);
                    }
                }

                // add the right, if the right neighbour is a number element
                if let Some(n) = neighbours.1 {
                    if let Element::Number(nr) = n {
                        *n = Element::Number(*nr + add_right);
                    }
                }

                right_exploded = true;
            }

            if left_exploded {
                // if we exploded the left, replace the left with a number 0
                self.left = Element::Number(0);
            }
            if right_exploded {
                // if we exploded the right, replace the right with a number 0
                self.right = Element::Number(0);
            }

            left_exploded || right_exploded
        } else {
            // if we are not at the fourth level, we need bubble explosion down
            // to the next level

            // keep track of what exploded, and return if it happened, so we don't explode twice.
            let mut left_exploded = false;
            let mut right_exploded = false;

            if let Element::Pair(p) = &mut self.left {
                left_exploded = p.explode(level + 1, (neighbours.0, self.right.leftmost_number()));
            }
            if left_exploded {
                return true;
            }

            if let Element::Pair(p) = &mut self.right {
                right_exploded = p.explode(level + 1, (self.left.rightmost_number(), neighbours.1));
            }
            if right_exploded {
                return true;
            }

            // nothing exploded below us.
            false
        }
    }

    /// Perform a split on the number.
    fn split(&mut self) -> bool {
        // If any regular number is 10 or greater, the leftmost such regular number splits.
        // To split a regular number, replace it with a pair; the left element of the pair should be the regular number
        //  divided by two and rounded down, while the right element of the pair should be the regular number divided
        // by two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
        // We only should split once per number, so we should return true if we have split.

        // To work from left to right we follow left numbers and left pairs first.
        if let Element::Number(n) = &self.left {
            if *n >= 10 {
                self.left = Element::Pair(Box::new(Snailfish {
                    left: Element::Number(*n / 2),
                    right: Element::Number((*n + 1) / 2),
                }));
                return true;
            }
        }
        let mut left_split = false;
        if let Element::Pair(p) = &mut self.left {
            left_split = p.split();
        }
        if left_split {
            return true;
        }

        // Then we follow right numbers and right pairs.
        if let Element::Number(n) = &self.right {
            if *n >= 10 {
                self.right = Element::Pair(Box::new(Snailfish {
                    left: Element::Number(*n / 2),
                    right: Element::Number((*n + 1) / 2),
                }));
                return true;
            }
        }

        let mut right_split = false;
        if let Element::Pair(p) = &mut self.right {
            right_split = p.split();
        }
        if right_split {
            return true;
        }

        false
    }

    fn leftmost_number(&mut self) -> Option<&mut Element> {
        // if the left is a number, return it
        if let Element::Number(_) = &self.left {
            return Some(&mut self.left);
        }
        // if the left is a pair, return the leftmost number
        if let Element::Pair(p) = &mut self.left {
            return p.leftmost_number();
        }
        // if we get here, we have a problem
        None
    }

    fn rightmost_number(&mut self) -> Option<&mut Element> {
        // if the right is a number, return it
        if let Element::Number(_) = &self.right {
            return Some(&mut self.right);
        }
        // if the right is a pair, return the rightmost number
        if let Element::Pair(p) = &mut self.right {
            return p.rightmost_number();
        }
        // if we get here, we have a problem
        None
    }

    fn magnitude(&self) -> i32 {
        // The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude
        // of its right element. The magnitude of a regular number is just that number.
        let left = match &self.left {
            Element::Number(n) => *n,
            Element::Pair(p) => p.magnitude(),
        };
        let right = match &self.right {
            Element::Number(n) => *n,
            Element::Pair(p) => p.magnitude(),
        };
        3 * left + 2 * right
    }
}

pub fn largest_magnitude(nrs: &[Snailfish]) -> i32 {
    // find the largest magnitude of any number in the array
    let magnitude = 0;
    let magnitudes: Vec<i32> = nrs
        .iter()
        .permutations(2)
        .map(|sv| {
            let sum = sv[0].clone() + sv[1].clone();
            sum.magnitude()
        })
        .collect();

    *magnitudes.iter().max().unwrap_or(&magnitude)
}

pub fn main() {
    let strings = read_strs("input/day18.txt");

    let number = strings
        .iter()
        .map(|s| Snailfish::from_str(s).unwrap())
        .reduce(|a, i| a + i)
        .unwrap();

    println!("Magnitude of final sum = {}", number.magnitude());

    let largest_magnitude = largest_magnitude(
        &strings
            .iter()
            .map(|s| Snailfish::from_str(s).unwrap())
            .collect::<Vec<Snailfish>>(),
    );

    println!("Largest magnitude = {}", largest_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fromstr_snailfish_numbers() {
        let str = "[1,2]";
        let number: Snailfish = str.parse().unwrap();

        assert_eq!(number.left, Element::Number(1));
        assert_eq!(number.right, Element::Number(2));
    }

    #[test]
    fn test_fromstr_snailfish_pairs() {
        let str = "[[1,2],[3,4]]";
        let number: Snailfish = str.parse().unwrap();

        if let Element::Pair(pair) = number.left {
            assert_eq!(pair.left, Element::Number(1));
            assert_eq!(pair.right, Element::Number(2));
        } else {
            panic!("Left element is not a pair");
        }

        if let Element::Pair(pair) = number.right {
            assert_eq!(pair.left, Element::Number(3));
            assert_eq!(pair.right, Element::Number(4));
        } else {
            panic!("Right element is not a pair");
        }
    }

    #[test]
    /// test parsing and displaying a snailfish number
    fn test_snailfish_display() {
        let str = "[1,2]";
        let number: Snailfish = str.parse().unwrap();
        let disp = format!("{}", number);
        assert_eq!(disp, str);

        let str = "[[1,2],[3,4]]";
        let number: Snailfish = str.parse().unwrap();
        let disp = format!("{}", number);
        assert_eq!(disp, str);
    }

    #[test]
    /// test adding two snailfish numbers
    fn test_snailfish_add() {
        let str1 = "[1,2]";
        let number1: Snailfish = str1.parse().unwrap();
        let str2 = "[[3,4],5]";
        let number2: Snailfish = str2.parse().unwrap();

        let sum = number1 + number2;

        let str = "[[1,2],[[3,4],5]]";
        let sumstr = format!("{}", sum);

        assert_eq!(sumstr, str);
    }

    #[test]
    /// test needs_exploding
    fn test_snailfish_needs_exploding() {
        let str = "[[[[[9,8],1],2],3],4]";
        let number: Snailfish = str.parse().unwrap();
        assert!(number.needs_exploding(1));

        let str = "[[[[0,9],2],3],4]";
        let number: Snailfish = str.parse().unwrap();
        assert!(!number.needs_exploding(1));

        let str = "[7,[6,[5,[4,[3,2]]]]]";
        let number: Snailfish = str.parse().unwrap();
        assert!(number.needs_exploding(1));

        let str = "[7,[6,[5,[7,0]]]]";
        let number: Snailfish = str.parse().unwrap();
        assert!(!number.needs_exploding(1));
    }

    #[test]
    /// test needs_splitting
    fn test_snailfish_needs_splitting() {
        let number = Snailfish::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        assert!(number.needs_splitting());

        let number = Snailfish::from_str("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
        assert!(!number.needs_splitting());
    }

    #[test]
    /// test exploding
    fn test_explode() {
        let mut number = Snailfish::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[[[[0,9],2],3],4]");

        let mut number = Snailfish::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[7,[6,[5,[7,0]]]]");

        let mut number = Snailfish::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        // [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]]
        let mut number = Snailfish::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        assert!(number.needs_exploding(1));
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_split() {
        let mut number = Snailfish::from_str("[10,0]").unwrap();
        number.split();
        assert_eq!(number.to_string(), "[[5,5],0]");

        let mut number = Snailfish::from_str("[11,0]").unwrap();
        number.split();
        assert_eq!(number.to_string(), "[[5,6],0]");

        let mut number = Snailfish::from_str("[0,10]").unwrap();
        number.split();
        assert_eq!(number.to_string(), "[0,[5,5]]");

        let mut number = Snailfish::from_str("[0,11]").unwrap();
        number.split();
        assert_eq!(number.to_string(), "[0,[5,6]]");
    }

    #[test]
    fn test_split_bug() {
        let mut number = Snailfish::from_str("[19,20]").unwrap();
        number.split();
        assert_eq!(number.to_string(), "[[9,10],20]");
        number.split();
        assert_eq!(number.to_string(), "[[9,[5,5]],20]");
        number.split();
        assert_eq!(number.to_string(), "[[9,[5,5]],[10,10]]");
        number.split();
        assert_eq!(number.to_string(), "[[9,[5,5]],[[5,5],10]]");
        number.split();
        assert_eq!(number.to_string(), "[[9,[5,5]],[[5,5],[5,5]]]");
    }

    #[test]
    fn test_successive_operations() {
        // init number as: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
        let mut number = Snailfish::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();

        // after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        // after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");

        // after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
        number.split();
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        // after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
        number.split();
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        // after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
        number.explode(1, (None, None));
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_reduce() {
        let n1 = Snailfish::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let n2 = Snailfish::from_str("[1,1]").unwrap();
        let number = n1 + n2;

        // Once no reduce actions apply, the snailfish number that remains is the actual
        // result of the addition operation: [[[[0,7],4],[[7,8],[6,0]]],[8,1]].
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_add_and_reduce_1() {
        // For example, the final sum of this list is [[[[1,1],[2,2]],[3,3]],[4,4]]:
        // [1,1]
        // [2,2]
        // [3,3]
        // [4,4]
        let n1 = Snailfish::from_str("[1,1]").unwrap();
        let n2 = Snailfish::from_str("[2,2]").unwrap();
        let n3 = Snailfish::from_str("[3,3]").unwrap();
        let n4 = Snailfish::from_str("[4,4]").unwrap();
        let number = n1 + n2 + n3 + n4;

        // the final sum of this list is [[[[1,1],[2,2]],[3,3]],[4,4]]:
        assert_eq!(number.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_add_and_reduce_2() {
        // The final sum of this list is [[[[3,0],[5,3]],[4,4]],[5,5]]:
        // [1,1]
        // [2,2]
        // [3,3]
        // [4,4]
        // [5,5]
        let n1 = Snailfish::from_str("[1,1]").unwrap();
        let n2 = Snailfish::from_str("[2,2]").unwrap();
        let n3 = Snailfish::from_str("[3,3]").unwrap();
        let n4 = Snailfish::from_str("[4,4]").unwrap();
        let n5 = Snailfish::from_str("[5,5]").unwrap();
        let number = n1 + n2 + n3 + n4 + n5;

        // the final sum of this list is [[[[3,0],[5,3]],[4,4]],[5,5]]:
        assert_eq!(number.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_add_and_reduce_3() {
        // The final sum of this list is [[[[5,0],[7,4]],[5,5]],[6,6]]:
        // [1,1]
        // [2,2]
        // [3,3]
        // [4,4]
        // [5,5]
        // [6,6]
        let n1 = Snailfish::from_str("[1,1]").unwrap();
        let n2 = Snailfish::from_str("[2,2]").unwrap();
        let n3 = Snailfish::from_str("[3,3]").unwrap();
        let n4 = Snailfish::from_str("[4,4]").unwrap();
        let n5 = Snailfish::from_str("[5,5]").unwrap();
        let n6 = Snailfish::from_str("[6,6]").unwrap();
        let number = n1 + n2 + n3 + n4 + n5 + n6;

        // the final sum of this list is [[[[5,0],[7,4]],[5,5]],[6,6]]:
        assert_eq!(number.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn a_slightly_larger_example() {
        let sum = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .iter()
        .map(|s| Snailfish::from_str(s).unwrap())
        .reduce(|a, i| a + i)
        .unwrap();

        // The final sum [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] is found after
        // adding up the above snailfish numbers:
        assert_eq!(
            sum.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn a_slightly_larger_example_step_by_step() {
        let n1 = Snailfish::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap();
        let n2 = Snailfish::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap();
        let sum = n1 + n2;
        assert_eq!(
            sum.to_string(),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );

        let n3 = Snailfish::from_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]").unwrap();
        let sum = sum + n3;
        assert_eq!(
            sum.to_string(),
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        );

        let n4 =
            Snailfish::from_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]").unwrap();
        let sum = sum + n4;
        assert_eq!(
            sum.to_string(),
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
        );

        let n5 = Snailfish::from_str("[7,[5,[[3,8],[1,4]]]]").unwrap();
        let sum = sum + n5;
        assert_eq!(
            sum.to_string(),
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
        );
    }

    #[test]
    fn why_not() {
        let n1 =
            Snailfish::from_str("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
                .unwrap();
        let n2 = Snailfish::from_str("[7,[5,[[3,8],[1,4]]]]").unwrap();
        let sum = n1 + n2;
        assert_eq!(
            sum.to_string(),
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        let n1 = Snailfish::from_str("[9,1]").unwrap();
        assert_eq!(n1.magnitude(), 29);

        let n2 = Snailfish::from_str("[1,9]").unwrap();
        assert_eq!(n2.magnitude(), 21);

        let n3 = Snailfish::from_str("[[9,1],[1,9]]").unwrap();
        assert_eq!(n3.magnitude(), 129);

        // Here are a few more magnitude examples:

        // [[1,2],[[3,4],5]] becomes 143.
        let n4 = Snailfish::from_str("[[1,2],[[3,4],5]]").unwrap();
        assert_eq!(n4.magnitude(), 143);

        // [[[[0,7],4],[[7,8],[6,0]]],[8,1]] becomes 1384.
        let n5 = Snailfish::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(n5.magnitude(), 1384);

        // [[[[1,1],[2,2]],[3,3]],[4,4]] becomes 445.
        let n6 = Snailfish::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(n6.magnitude(), 445);

        // [[[[3,0],[5,3]],[4,4]],[5,5]] becomes 791.
        let n7 = Snailfish::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(n7.magnitude(), 791);

        // [[[[5,0],[7,4]],[5,5]],[6,6]] becomes 1137.
        let n8 = Snailfish::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(n8.magnitude(), 1137);

        // [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] becomes 3488.
        let n9 =
            Snailfish::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(n9.magnitude(), 3488);
    }

    #[test]
    fn homework_assignment() {
        let sum = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .map(|s| Snailfish::from_str(s).unwrap())
        .reduce(|a, i| a + i)
        .unwrap();

        // The final sum [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] is found after
        // adding up the above snailfish numbers:
        assert_eq!(
            sum.to_string(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );

        // The magnitude of the final sum is: 4140
        assert_eq!(sum.magnitude(), 4140);
    }

    #[test]
    fn test_largest_magnitude() {
        let nrs = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .map(|s| Snailfish::from_str(s).unwrap())
        .collect::<Vec<Snailfish>>();

        assert_eq!(largest_magnitude(&nrs), 3993);
    }
}
