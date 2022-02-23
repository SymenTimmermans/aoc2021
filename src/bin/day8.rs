use std::collections::HashMap;

use aoc2021::read_strs;

/// Count the number of times a digit 1, 4, 7 or 8 appears in the output.
/// This can be simplified to:
/// 2 letters -> 1
/// 3 letters -> 7
/// 4 letters -> 4
/// 7 letters -> 8
/// So actually, we should count the number of times those length strings are in the output.
fn day8() {
    let mut lengths = vec![0; 8];
    // read in the input file as a vector of strings
    let input = read_strs("input/day8.txt");
    for line in input {
        if let Some((_patterns, output)) = line.split_once("|") {
            output.split_whitespace().for_each(|c| {
                lengths[c.len()] += 1;
            });
        }
    }

    println!("{:?}", lengths);
    // print the sum of elements 2, 3, 4, and 7 in the vector
    println!(
        "1,4,7 and 8 appear {} times",
        lengths[2] + lengths[3] + lengths[4] + lengths[7]
    );
}

fn day8b() {
    let input = read_strs("input/day8.txt");
    let mut sum = 0;

    for line in input {
        if let Some((patterns, output)) = line.split_once("|") {
            let value = determine_value(patterns, output);
            println!("Value: {}", value);
            sum += value;
        }
    }

    println!("Sum: {}", sum);
}

/// We will use an algorithm that looks at a pattern,
/// and adds the length of the pattern +1 to a hashmap for each letter in the pattern.
///
/// This will give us a hashmap that should have the following values:
/// segment      |  value
/// -------------------------
/// top          | 51
/// top left     | 40
/// top right    | 46
/// middle       | 45
/// bottom left  | 28
/// bottom right | 53
/// bottom       | 74
///
/// Following these values for each segment, we can add up the values for each char in a pattern and it should
/// correspond to a certain total for each digit:
///
/// 1: 99
/// 2: 217
/// 3: 242
/// 4: 184
/// 5: 236
/// 6: 264
/// 7: 150
/// 8: 310
/// 9: 282
/// 0: 265
///
/// We only have to loop trough the output values and calculate the sum of the values for each digit, and match that
/// sum to a given digit. We accumulate the digits in the output and turn this into a u32.
///
fn determine_value(patterns: &str, output: &str) -> u32 {
    let mut map: HashMap<char, usize> = HashMap::new();
    let patterns = patterns.split_whitespace().collect::<Vec<&str>>();

    // add the values for the segments to the hashmap
    for pattern in patterns.iter() {
        for c in pattern.chars() {
            *map.entry(c).or_insert(0) += pattern.len() + 1;
        }
    }

    // now we can loop through the output and calculate the value for each digit
    let output = output.split_whitespace().collect::<Vec<&str>>();
    let mut number: Vec<char> = Vec::new();

    for c in output.iter() {
        let mut sum = 0;
        for d in c.chars() {
            sum += map[&d];
        }

        number.push(match sum {
            99 => '1',
            217 => '2',
            242 => '3',
            184 => '4',
            236 => '5',
            264 => '6',
            150 => '7',
            310 => '8',
            282 => '9',
            265 => '0',
            _ => ' ',
        });
    }

    number.iter().collect::<String>().parse::<u32>().unwrap()
}

pub fn main() {
    day8();
    day8b();
}