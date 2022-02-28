use aoc2021::read_strs;

fn gamma_rate(numbers: &[String]) -> (i32, i32) {
    let mut result = (String::new(), String::new());
    let positions = numbers[0].len();
    for i in 0..positions {
        let mut counts = vec![0; 2];
        for number in numbers.iter() {
            let digit = number.chars().nth(i).unwrap();
            counts[digit as usize - '0' as usize] += 1;
        }
        if counts[0] > counts[1] {
            result.0.push('0');
            result.1.push('1');
        } else {
            result.0.push('1');
            result.1.push('0');
        }
    }
    (
        i32::from_str_radix(result.0.as_str(), 2).unwrap(),
        i32::from_str_radix(result.1.as_str(), 2).unwrap(),
    )
}

fn day3() {
    let numbers = read_strs("input/day3.txt");
    let (gamma_rate, epsilon) = gamma_rate(&numbers);
    println!("Gamma {}", gamma_rate);
    println!("Epsil {}", epsilon);
    println!("Power {}", gamma_rate * epsilon);
}

fn rating_finder(numbers: &[String], most_common: bool) -> i32 {
    let mut pos = 0;
    let mut candidates = numbers.to_owned();
    while candidates.len() > 1 {
        // find the most common digit in the current position
        let mut counts = vec![0; 2];
        for number in candidates.iter() {
            let digit = number.chars().nth(pos).unwrap();
            counts[digit as usize - '0' as usize] += 1;
        }
        // figure out which digit to search for
        let digit;
        if most_common {
            digit = if counts[0] > counts[1] { '0' } else { '1' };
        } else {
            digit = if counts[0] > counts[1] { '1' } else { '0' };
        }

        // remove all numbers that don't match the digit at
        // the current position
        candidates = candidates
            .iter()
            .filter(|number| number.chars().nth(pos).unwrap() == digit)
            .cloned()
            .collect();

        pos += 1;
    }

    i32::from_str_radix(candidates[0].as_str(), 2).unwrap()
}

fn ox_rating(numbers: &[String]) -> i32 {
    rating_finder(numbers, true)
}

fn co2_rating(numbers: &[String]) -> i32 {
    rating_finder(numbers, false)
}

fn day3b() {
    let numbers = read_strs("input/day3.txt");
    let ox_rating = ox_rating(&numbers);
    let co2_rating = co2_rating(&numbers);

    println!("Ox  rating {}", ox_rating);
    println!("CO2 rating {}", co2_rating);
    println!("LS  rating {}", ox_rating * co2_rating);
}

pub fn main() {
    day3();
    day3b();
}
