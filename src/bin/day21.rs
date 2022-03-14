struct Die {
    sides: usize,
    last: usize,
    rolls: usize,
}

impl Die {
    fn new(sides: usize) -> Die {
        Die {
            sides,
            last: 0,
            rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.take(3).sum()
    }
}

// implement iterator for Die
impl Iterator for Die {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = if self.last == self.sides {
            1
        } else {
            self.last + 1
        };
        self.rolls += 1;
        Some(self.last)
    }
}

const BOARD_SIZE: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Player {
        Player { pos, score: 0 }
    }

    fn turn(&mut self, die: &mut Die) {
        // roll the die
        let roll = die.roll();
        // move the player
        self.do_move(roll);
    }

    fn do_move(&mut self, roll: usize) {
        self.pos = (self.pos + roll - 1) % BOARD_SIZE + 1;

        // add the position to the current score
        self.score += self.pos;
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }

    fn has_won_2(&self) -> bool {
        self.score >= 21
    }
}

/// Each time you roll the dice three times, you split into 27 universes, but
/// the total of the dice is only between 3 and 9. We combine those cases,
/// because we nr_u the counter along anyways.
#[rustfmt::skip]
const QUANTUM_THROWS: [(usize, u128); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn recurse_game(p1: &Player, p2: &Player, nr_u: u128) -> (u128, u128) {
    // println!(
    //     "[ {} ] --- P1 score: {}, P2 score: {}",
    //     nr_u, p1.score, p2.score
    // );

    // Do a turn for p1.
    let win_universes = QUANTUM_THROWS
        .iter()
        .map(|(value, universes)| {
            // clone player 1
            let mut p1_clone = *p1;
            // move the player based on the value thrown
            p1_clone.do_move(*value);

            // println!("[ {} ] --- P1 score now {}", nr_u, p1_clone.score);

            // if p1 has won, return the universes.
            if p1_clone.has_won_2() {
                // println!(
                //     "[ {} ] --- P1 wins in {} universes, * nr_u = {}",
                //     nr_u,
                //     universes,
                //     nr_u * universes
                // );
                (*universes * nr_u, 0)
            } else {
                // give player 2 the turn
                let (p2wu, p1wu) = recurse_game(p2, &p1_clone, *universes);
                // println!(
                //     "[ {} ] --- came back from recursive cal, p2: {}, p1: {}",
                //     nr_u, p1wu, p2wu
                // );
                (p1wu * nr_u, p2wu * nr_u)
            }
        })
        .reduce(|(p1wu, p2wu), (p1wu2, p2wu2)| (p1wu + p1wu2, p2wu + p2wu2));

    println!("[ {} ] Win universes: {:?}", nr_u, win_universes);

    (win_universes.unwrap().0, win_universes.unwrap().1)
}

fn main() {
    // create a new die
    let mut die = Die::new(100);

    // create player 1 and player 2
    let mut p1 = Player::new(7);
    let mut p2 = Player::new(3);

    loop {
        p1.turn(&mut die);

        if p1.has_won() {
            println!("Player 1 wins!");
            break;
        }

        p2.turn(&mut die);

        if p2.has_won() {
            println!("Player 2 wins!");
            break;
        }
    }

    let loser_score = if p1.has_won() { p2.score } else { p1.score };

    println!("The loser scored {}", loser_score);

    // multiply loser score by die rolls and print out that number
    println!("Part 1: Game outcome: {}", loser_score * die.rolls);

    // Part 2
    // ------
    //
    // Here's where the craziness happens. Of course, we're turning the problem entirely upside down
    // by introducing a "quantum" die. The "die" splits the universe into 3 universes, each with a thrown value
    // of 1, 2, or 3.
    // This means we should generalize the problem. The numbers of universes mentioned are far too big to be
    // iterating through in a for loop.
    //
    // Maybe we should first determine the number of throws that definetly ends the game.
    // The game ends when the player's score is 21 or higher.
    //
    // Maybe we should just try recursion and see how far we get.
    //
    // make two new players:
    let p1 = Player::new(7);
    let p2 = Player::new(3);

    let (p1wu, p2wu) = recurse_game(&p1, &p2, 1);

    // printout p1wu and p2wu
    println!("Part 2: Game outcome: {} vs {}", p1wu, p2wu);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_die() {
        let mut die = Die::new(6);
        assert_eq!(die.next(), Some(1));
        assert_eq!(die.next(), Some(2));
        assert_eq!(die.next(), Some(3));
        assert_eq!(die.next(), Some(4));
        assert_eq!(die.next(), Some(5));
        assert_eq!(die.next(), Some(6));
        assert_eq!(die.next(), Some(1));
    }

    #[test]
    fn test_die_three_rolls() {
        let mut die = Die::new(6);
        assert_eq!(die.roll(), 6);
        assert_eq!(die.roll(), 15);
    }

    #[test]
    fn test_play_game() {
        let mut die = Die::new(100);

        let mut p1 = Player::new(4);
        let mut p2 = Player::new(8);

        p1.turn(&mut die);

        // player should move to position 10 and have a score of 10.
        assert_eq!(p1.pos, 10);
        assert_eq!(p1.score, 10);

        // Player 2 rolls 4+5+6 and moves to space 3 for a total score of 3.
        p2.turn(&mut die);
        assert_eq!(p2.pos, 3);
        assert_eq!(p2.score, 3);

        // Player 1 rolls 7+8+9 and moves to space 4 for a total score of 14.
        p1.turn(&mut die);
        assert_eq!(p1.pos, 4);
        assert_eq!(p1.score, 14);

        // Player 2 rolls 10+11+12 and moves to space 6 for a total score of 9.
        p2.turn(&mut die);
        assert_eq!(p2.pos, 6);
        assert_eq!(p2.score, 9);

        // Player 1 rolls 13+14+15 and moves to space 6 for a total score of 20.
        p1.turn(&mut die);
        assert_eq!(p1.pos, 6);
        assert_eq!(p1.score, 20);

        // Player 2 rolls 16+17+18 and moves to space 7 for a total score of 16.
        p2.turn(&mut die);
        assert_eq!(p2.pos, 7);
        assert_eq!(p2.score, 16);

        // Player 1 rolls 19+20+21 and moves to space 6 for a total score of 26.
        p1.turn(&mut die);
        assert_eq!(p1.pos, 6);
        assert_eq!(p1.score, 26);

        // Player 2 rolls 22+23+24 and moves to space 6 for a total score of 22.
        p2.turn(&mut die);
        assert_eq!(p2.pos, 6);
        assert_eq!(p2.score, 22);
    }

    #[test]
    fn test_game_end() {
        let mut die = Die::new(100);

        let mut p1 = Player::new(4);
        let mut p2 = Player::new(8);

        loop {
            p1.turn(&mut die);
            if p1.has_won() {
                break;
            }
            p2.turn(&mut die);
            if p2.has_won() {
                break;
            }
        }

        // player one should have won
        assert!(p1.has_won());

        // player one should have a score of over 1000
        assert!(p1.score >= 1000);

        // player two should have a score of 745
        assert_eq!(p2.score, 745);

        // the dice should have been rolled 993 times
        assert_eq!(die.rolls, 993);
    }

    #[test]
    fn test_recurse_game() {
        // Create two players where on is on the virge of winning.
        let mut p1 = Player::new(1);
        p1.score = 20;

        let mut p2 = Player::new(2);
        p2.score = 19;

        // in this situation, player 1 should always win, because it's his turn, and he will
        // score more than 21
        let (p1wu, p2wu) = recurse_game(&p1, &p2, 1);

        // player 1 throws the die 3 times, which splits the universe into 3 * 3 * 3 = 27 universes.
        // so p1wu should be 27
        assert_eq!(p1wu, 27);
        // p2 does not win, so p2wu should be 0
        assert_eq!(p2wu, 0);

        // print a line
        println!("-----------------------------------------------------");

        // lets try a different situation. Player 1 is not winning, but player 2 is.
        let mut p1 = Player::new(1);
        p1.score = 0;
        let mut p2 = Player::new(2);
        p2.score = 20;

        // in this situation, player 2 should always win, because player 1 can not win in one move.
        let (p1wu, p2wu) = recurse_game(&p1, &p2, 1);

        // player 1 throws the die 3 times, which splits the universe into 3 * 3 * 3 = 27 universes.
        // player 2 then does the same and wins, in all 27 * 27 universes.
        // so p1wu should be 0
        assert_eq!(p1wu, 0);
        // p2 wins after 27 * 27 universes
        assert_eq!(p2wu, 27 * 27);

        // print a line
        println!("-----------------------------------------------------");

        // another edge case. Player 1 is almost winning but can only win when he throws 1+1+1, which only
        // happens in 1 universe. In all other universes, player 2 will win.

        let mut p1 = Player::new(7);
        p1.score = 11;
        let mut p2 = Player::new(2);
        p2.score = 20;

        let (p1wu, p2wu) = recurse_game(&p1, &p2, 1);

        // player 1 should only win in 1 universe
        assert_eq!(p1wu, 1);
        // player 2 should win in 26 * 27 universes
        assert_eq!(p2wu, 26 * 27);
    }

    //  #[test]
    fn test_recurse_game_example() {
        let p1 = Player::new(4);
        let p2 = Player::new(8);

        let (p1wu, p2wu) = recurse_game(&p1, &p2, 1);

        // Using the same starting positions as in the example above, player 1 wins in 444356092776315 universes,
        assert_eq!(p1wu, 444356092776315);
        // while player 2 merely wins in 341960390180808 universes.
        assert_eq!(p2wu, 341960390180808);
    }
}
