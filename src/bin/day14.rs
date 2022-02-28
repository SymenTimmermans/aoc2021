use std::collections::HashMap;

use aoc2021::read_strs;

struct Instructions {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Instructions {
    pub fn from_file(file_path: &str) -> Self {
        let lines = read_strs(file_path);

        let template = lines
            .iter()
            .find(|l| !l.is_empty() && !(*l).contains("->"))
            .unwrap()
            .chars()
            .collect();

        let rules = lines
            .iter()
            .filter(|l| !l.is_empty() && (*l).contains("->"))
            .map(|l| {
                let mut split = l.split(" -> ");
                let left = split.next().unwrap().trim().chars().collect::<Vec<_>>();
                let right = split.next().unwrap().trim().chars().next().unwrap();
                ((left[0], left[1]), right)
            })
            .collect();

        Instructions { template, rules }
    }

    /// Apply x steps to the start template
    pub fn steps(&self, nr: usize) -> String {
        let mut start = self.template.clone();

        for _ in 0..nr {
            start = self.apply(&start);
        }

        start.iter().collect()
    }

    /// this algorithm loops through the indexes of the string
    /// we should return, and determines each character by the
    /// following rules
    fn apply(&self, start: &[char]) -> Vec<char> {
        let mut result = vec![];
        let len = start.len() * 2 - 1;

        for i in 0..len {
            if i % 2 == 0 {
                result.push(start[i / 2]);
            } else {
                // odd number, we should be able to read the left and right
                // chars from the start
                let left = start[(i - 1) / 2];
                let right = start[(i + 1) / 2];
                // we should be able to find the rule for this
                let rule = self.rules.get(&(left, right)).unwrap();
                result.push(*rule);
            }
        }

        result
    }

    // this algorithm, we don't keep track of the string we're building
    // but only counting pairs. So we don't return the string, but return
    // the polymer score directly.
    pub fn steps_2(&self, nr: usize) -> u64 {
        type Pair = (char, char);
        // pairs that don't have replacement rules can't be handled,
        // so we can instantiate the hashmap with the pairs we know.
        let mut pairs = HashMap::<Pair, usize>::new();

        // initialize the counts for the pairs in the template.
        for i in 0..(self.template.len() - 1) {
            let pair = (self.template[i], self.template[(i + 1)]);
            pairs.insert(pair, 1);
        }

        self.apply_2(&mut pairs, nr);
        
        // score the pairs hashmap,
        Self::score_hashmap(&pairs)
    }

    /// Apply the folding through an already initialized hashmap
    fn apply_2(&self, pairs: &mut HashMap<(char, char), usize>, nr: usize) {
        let mut newpairs = HashMap::<(char, char), usize>::new();

        for _ in 0..nr {
            // loop through the pairs, and decrease the count for each
            // pair that is in the start.
            for (pair, count) in pairs.iter_mut() {
                // get the folded pairs for this pair
                self.get_folded_pairs(*pair).iter().for_each(|p| {
                    // increase the value in newpairs with count
                    *newpairs.entry(*p).or_insert(0) += *count;
                });
                *count = 0;
            }

            // add all counts in newpairs to the pairs in pairs
            pairs.extend(newpairs.into_iter());
            
            // clear newpairs 
            newpairs = HashMap::<(char, char), usize>::new();
        }
    }

    // return a vec of pairs that are added if we fold a certain pair
    pub fn get_folded_pairs(&self, pair: (char, char)) -> Vec<(char, char)> {
        // find the pair in the rules map
        let nc = self.rules.get(&pair).unwrap();
        vec![(pair.0, *nc), (*nc, pair.1)]
    }

    fn score_hashmap(pairs: &HashMap<(char, char), usize>) -> u64 {
        // make a hashmap that holds each char
        let mut chars_front = HashMap::<char, usize>::new();
        let mut chars_back = HashMap::<char, usize>::new();

        // loop through the pairs, and increase the count for each
        // char in the pairs
        for (pair, count) in pairs.iter() {
            *chars_front.entry(pair.0).or_insert(0) += *count;
            *chars_back.entry(pair.1).or_insert(0) += *count;
        }

        // for each char, get the max of the counts in front_and back.
        let chars = chars_front
            .keys()
            .chain(chars_back.keys())
            .collect::<Vec<&char>>();
        let mut char_counts = HashMap::<char, usize>::new();
        for c in chars {
            let max = chars_front
                .get(c)
                .unwrap_or(&0)
                .max(chars_back.get(c).unwrap_or(&0));
            // insert max in char_counts
            char_counts.insert(*c, *max);
        }

        // find the char with the lowest count
        let lowest = char_counts
            .iter()
            .filter(|(_, c)| **c > 0)
            .min_by_key(|(_, c)| *c).unwrap();
        // find the char with the highest count
        let highest = char_counts
            .iter()
            .filter(|(_, c)| **c > 0)
            .max_by_key(|(_, c)| *c).unwrap();

        // subtract the lowest count from the highest count
        *highest.1 as u64 - *lowest.1 as u64
    }

    /// this function scores the polymer.
    /// it takes the count of the most occurring char
    /// and subtracts the count of the least occurring char
    /// and returns that
    fn score(polymer: &str) -> usize {
        let mut counts = HashMap::new();

        for c in polymer.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut most_occurring = None;
        let mut least_occurring = None;

        for (c, count) in counts.iter() {
            if most_occurring.is_none() || *count > counts[&most_occurring.unwrap()] {
                most_occurring = Some(*c);
            }

            if least_occurring.is_none() || *count < counts[&least_occurring.unwrap()] {
                least_occurring = Some(*c);
            }
        }

        counts[&most_occurring.unwrap()] - counts[&least_occurring.unwrap()]
    }
}

pub fn main() {
    let instr = Instructions::from_file("input/day14.txt");

    let polymer = instr.steps(10);

    println!("Score: {}", Instructions::score(&polymer));

    // For part 2, we need to do 40 steps, which slows down the process.
    // We need to change tha algorithm to use a different approach
    // to generating the polymer.
    // Any approach that keeps the entire polymer string in memory will
    // not work. it will take way too much memory to generate the polymer.
    // is there a smarter way to do this?
    // since we only care about the score of the most and least common
    // elements, do we really need to have the entire polymer?
    // can we find shortcuts to reduce the memory footprint?
    // should we actually use recursion to generate the polymer?
    // maybe we can use a stack to store the positions of the most and
    // least common elements.

    // let's look at what actually hapoens when we create a polymer.
    // We start with an element pair, like NN, which folds to include a
    // letter C for instance. this means, that in the next run, we have
    // two pairs, NC and CN
    // Really, the order of the pairs shouldn't matter.
    // Lets see if this is correct for the example.
    // Starting with NNCB, we have three pairs:
    // NN, NC, CB
    // following the rules would give us new pairs:
    // NN -> NC + CN
    // NC -> NB + BC
    // CB -> CH + HB
    // If we make a hashmap containing all possible pairs, we can
    // just look up the pairs in the hashmap and
    // - decrease the source pair by one,
    // - increase both target pairs by one.
    // After X iterations, we should count the individual letters of the
    // pairs to determine the score.
    let polymer = instr.steps_2(40);

    println!("Score: {}", polymer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_file() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        assert_eq!(instr.template, vec!['N', 'N', 'C', 'B']);

        assert_eq!(instr.rules.len(), 16);

        assert_eq!(instr.rules[&('C', 'H')], 'B');
        assert_eq!(instr.rules[&('B', 'H')], 'H');
        assert_eq!(instr.rules[&('C', 'N')], 'C');
    }

    #[test]
    fn test_steps() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        assert_eq!(instr.steps(0), "NNCB".to_string());
        assert_eq!(instr.steps(1), "NCNBCHB".to_string());
        assert_eq!(instr.steps(2), "NBCCNBBBCBHCB".to_string());
        assert_eq!(instr.steps(3), "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string());
        assert_eq!(
            instr.steps(4),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string()
        );
    }

    #[test]
    fn test_folded_pairs() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        assert_eq!(
            instr.get_folded_pairs(('N', 'N')),
            vec![('N', 'C'), ('C', 'N')]
        );
        assert_eq!(
            instr.get_folded_pairs(('C', 'H')),
            vec![('C', 'B'), ('B', 'H')]
        );
    }


    #[test]
    fn test_apply_2() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        let mut hm = HashMap::new();
        hm.insert(('N', 'N'), 1);

        instr.apply_2(&mut hm, 1);

        assert_eq!(hm.len(), 3);
        assert_eq!(hm[&('N', 'C')], 1);
        assert_eq!(hm[&('C', 'N')], 1);
        assert_eq!(hm[&('N', 'N')], 0);

        hm = HashMap::new();
        hm.insert(('N', 'N'), 1);
        hm.insert(('N', 'C'), 1);
        hm.insert(('C', 'B'), 1);

        instr.apply_2(&mut hm, 1);

        println!("HM2: {:?}", hm);

        assert_eq!(hm[&('N', 'C')], 1);
        assert_eq!(hm[&('C', 'N')], 1);
        assert_eq!(hm[&('N', 'B')], 1);
        assert_eq!(hm[&('B', 'C')], 1);
        assert_eq!(hm[&('C', 'H')], 1);
        assert_eq!(hm[&('H', 'B')], 1);
    }


    #[test]
    fn test_steps2() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        assert_eq!(instr.steps_2(0), 1);
        assert_eq!(instr.steps_2(1), 1);
        assert_eq!(instr.steps_2(2), 5);

        assert_eq!(instr.steps_2(10), 1588);
    }

    #[test]
    fn test_occurrence_score() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        let polymer = instr.steps(10);

        assert_eq!(Instructions::score(&polymer), 1588);
    }

    #[test]
    fn test_more_steps() {
        let instr = Instructions::from_file("input/day14_ex.txt");

        assert_eq!(instr.steps_2(40), 2188189693529);
    }
}
