use aoc2021::read_strs;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<i32>>,
    won: bool,
}

impl Board {
    fn read(input: &[String]) -> Self {
        let mut tiles = Vec::new();
        for line in input {
            let mut row = Vec::new();
            // split the line on whitespace
            // and add numbers to the row
            for num in line.split_whitespace() {
                row.push(num.parse().unwrap());
            }
            tiles.push(row);
        }
        Board { tiles, won: false }
    }

    fn wins(&self, draws: &[i32]) -> bool {
        // figure out if any of the rows has all numbers in the draws
        for row in &self.tiles {
            if row.iter().all(|&x| draws.contains(&x)) {
                return true;
            }
        }

        // figure out if any of the columns has all numbers in the draws
        for i in 0..BOARD_SIZE {
            let mut column = Vec::new();
            for j in 0..BOARD_SIZE {
                column.push(self.tiles[j][i]);
            }
            if column.iter().all(|&x| draws.contains(&x)) {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self, draws: &[i32]) -> i32 {
        let mut sum = 0;
        for row in &self.tiles {
            for &num in row {
                if !draws.contains(&num) {
                    sum += num;
                }
            }
        }
        sum
    }
}

fn read_bingo(path: &str) -> (Vec<i32>, Vec<Board>) {
    let lines = read_strs(path);

    // first line contains the draws
    let draws: Vec<i32> = lines[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // the rest are the boards
    let mut boards = Vec::<Board>::new();
    let mut line = 2;
    while line < lines.len() {
        let board = Board::read(&lines[line..line + 5]);

        boards.push(board);
        line += 6;
    }

    (draws, boards)
}

fn day4() {
    let (draws, boards) = read_bingo("input/day4.txt");

    for i in 0..draws.len() {
        // slice of draws till now
        let draws_slice = draws[..i + 1].to_vec();
        // go through boards
        for board in boards.iter() {
            // check if the board is valid
            if board.wins(&draws_slice) {
                println!("{:?}", draws_slice);
                println!("{:?} wins", board);

                let sum_unmarked = board.sum_unmarked(&draws_slice);
                println!("sum of unmarked numbers: {}", sum_unmarked);
                println!("score: {}", sum_unmarked * draws[i]);
                return;
            }
        }
    }
}

fn day4b() {
    let (draws, mut boards) = read_bingo("input/day4.txt");

    let mut last_score = -1;

    for i in 0..draws.len() {
        // slice of draws till now
        let draws_slice = draws[..i + 1].to_vec();

        // go through boards
        for board in boards.iter_mut() {
            // skip boards that are already won
            if board.won {
                continue;
            }

            // check if the board is valid
            if board.wins(&draws_slice) {
                let sum_unmarked = board.sum_unmarked(&draws_slice);
                last_score = sum_unmarked * draws[i];
                board.won = true;
            }
        }
    }

    println!("last score: {}", last_score);
}

pub fn main() {
    day4();
    day4b();
}
