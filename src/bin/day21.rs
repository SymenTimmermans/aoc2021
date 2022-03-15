use std::cell::RefCell;
use std::collections::HashMap;

/// A deterministic die with N sides.
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

    /// Return a total score of three rolls.
    fn roll(&mut self) -> usize {
        self.take(3).sum()
    }
}

// implement iterator for Die
impl Iterator for Die {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = self.last % self.sides + 1;
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

//          p1 pos, p2 pos, p1 score, p2 score
type State = (usize, usize, usize, usize);

//                   p1wu, p2wu
type UniverseWins = (u128, u128);

thread_local! {
    static CACHE: RefCell<HashMap<(State, u128), UniverseWins>> = RefCell::new(HashMap::new());
}

fn recurse_game(init_state: State, nr_u: u128) -> UniverseWins {
    // check if we have already calculated this state
    if let Some(res) = CACHE.with(|cache| cache.borrow().get(&(init_state, nr_u)).cloned()) {
        return res;
    }

    // Do a turn for p1.
    let win_universes = QUANTUM_THROWS
        .iter()
        .map(|(value, universes)| {
            let mut state = (init_state.0, init_state.1, init_state.2, init_state.3);

            // move player 1
            state.0 = (state.0 + value - 1) % BOARD_SIZE + 1;
            // add the position to the current score
            state.2 += state.0;

            // if p1 has won, return the universes.
            if state.2 >= 21 {
                (*universes * nr_u, 0)
            } else {
                // give player 2 the turn, so flip the player data around.
                let state = (state.1, state.0, state.3, state.2);

                let (p2wu, p1wu) = recurse_game(state, *universes);
                (p1wu * nr_u, p2wu * nr_u)
            }
        })
        .reduce(|(p1wu, p2wu), (p1wu2, p2wu2)| (p1wu + p1wu2, p2wu + p2wu2))
        .expect("No win chancees!");

    // put this in the cache
    CACHE.with(|cache| {
        cache.borrow_mut().insert((init_state, nr_u), win_universes);
    });

    (win_universes.0, win_universes.1)
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
    // on position 7 and 3
    // with score 0
    let state = (7, 3, 0, 0);

    let (p1wu, p2wu) = recurse_game(state, 1);

    // printout p1wu and p2wu
    println!(
        "Part 2: Game outcome: {} vs {}, highest: {}",
        p1wu,
        p2wu,
        p1wu.max(p2wu)
    );
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
        let state = (1, 2, 20, 19);

        // in this situation, player 1 should always win, because it's his turn, and he will
        // score more than 21
        let (p1wu, p2wu) = recurse_game(state, 1);

        // player 1 throws the die 3 times, which splits the universe into 3 * 3 * 3 = 27 universes.
        // so p1wu should be 27
        assert_eq!(p1wu, 27);
        // p2 does not win, so p2wu should be 0
        assert_eq!(p2wu, 0);

        // print a line
        println!("-----------------------------------------------------");

        // lets try a different situation. Player 1 is not winning, but player 2 is.
        let state = (1, 2, 0, 20);

        // in this situation, player 2 should always win, because player 1 can not win in one move.
        let (p1wu, p2wu) = recurse_game(state, 1);

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

        let state = (7, 2, 11, 20);
        let (p1wu, p2wu) = recurse_game(state, 1);

        // player 1 should only win in 1 universe
        assert_eq!(p1wu, 1);
        // player 2 should win in 26 * 27 universes
        assert_eq!(p2wu, 26 * 27);
    }

    #[test]
    fn test_recurse_game_example() {
        let state = (4, 8, 0, 0);

        let (p1wu, p2wu) = recurse_game(state, 1);

        // Using the same starting positions as in the example above, player 1 wins in 444356092776315 universes,
        assert_eq!(p1wu, 444356092776315);
        // while player 2 merely wins in 341960390180808 universes.
        assert_eq!(p2wu, 341960390180808);
    }
}
