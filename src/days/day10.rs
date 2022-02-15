use super::read_strs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LineState {
    Valid,
    Incomplete(u64),
    Corrupt(char),
}

impl LineState {
    pub fn get_score(&self) -> u64 {
        match self {
            LineState::Valid => 0,
            LineState::Incomplete(score) => *score,
            LineState::Corrupt(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
        }
    }

    pub fn calc_completion_score(closers: &[char]) -> u64 {
        
        closers.iter().rev().map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }).fold(0, |acc, x| (acc * 5) + x)
    }
}

fn parse_line(str: &str) -> LineState {
    let mut closers: Vec<char> = vec![];
    // loop through each character in the string
    for c in str.chars() {
        match c {
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            '{' => closers.push('}'),
            '<' => closers.push('>'),
            _ => {
                if let Some(last) = closers.pop() {
                    if last != c {
                        return LineState::Corrupt(c);
                    }
                } else {
                    return LineState::Corrupt(c);
                }
            }
        }
    }
    if !closers.is_empty() {
        return LineState::Incomplete(LineState::calc_completion_score(&closers));
    }
    LineState::Valid
}

fn is_corrupt(state: LineState) -> bool {
    matches!(state, LineState::Corrupt(_))
}

pub fn day10() {
    let lines = read_strs("input/day10.txt");

    // print the number of total lines
    println!("  lines: {}", lines.len());

    let syntax_error_score: u64 = lines
        .iter()
        .map(|l| parse_line(l))
        .filter(|&s| matches!(s, LineState::Corrupt(_)))
        .map(|s| s.get_score())
        .sum();
    println!("syntax error score: {:?}", syntax_error_score);
}

pub fn day10b() {
    let lines = read_strs("input/day10.txt");

    // print the number of total lines
    println!("  lines: {}", lines.len());

    let mut completion_scores: Vec<u64> = lines
        .iter()
        .map(|l| parse_line(l))
        .filter(|&s| matches!(s, LineState::Incomplete(_)))
        .map(|s| s.get_score())
        .collect();


    // sort the completion scores
    completion_scores.sort_unstable();
    
    println!("completion scores: {:?}", completion_scores);

    // get middle value of completion scores vector
    let middle = completion_scores.len() / 2;
    let middle_score = completion_scores[middle];

    // print middle score
    println!("middle score: {}", middle_score);
        
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_completion_score() {
        assert_eq!(LineState::calc_completion_score(&[']', ')', '}', '>']), 294);
        // assert_eq!(LineState::calc_completion_score(&[']']), 2);
        // assert_eq!(LineState::calc_completion_score(&['}']), 3);
        // assert_eq!(LineState::calc_completion_score(&['>']), 4);
    }

}