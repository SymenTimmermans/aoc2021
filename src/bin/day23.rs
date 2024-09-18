use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{format, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Type {
    A,
    B,
    C,
    D,
}

impl Type {
    fn dest_burrow(&self) -> usize {
        match self {
            Type::A => 1,
            Type::B => 2,
            Type::C => 3,
            Type::D => 4,
        }
    }

    fn energy_cost(&self) -> usize {
        match self {
            Type::A => 1,
            Type::B => 10,
            Type::C => 100,
            Type::D => 1000,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Type::A => 'A',
            Type::B => 'B',
            Type::C => 'C',
            Type::D => 'D',
        }
    }
}

const HALLWAY_SIZE: usize = 11;
const BURROW_DEPTH: usize = 2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Position {
    Hallway(usize),
    Burrow(usize, usize),
}

impl Position {
    /// Return all possible positions
    fn all() -> Vec<Position> {
        let mut positions = Vec::new();
        for i in 0..HALLWAY_SIZE {
            positions.push(Position::Hallway(i));
        }
        for i in 1..=4 {
            for j in 1..=2 {
                positions.push(Position::Burrow(i, j));
            }
        }
        positions
    }

    /// Return the list of positions that lie between this position and the
    /// destination.
    fn path(&self, dest: Self) -> Vec<Position> {
        // if self and dest are the same return an empty list
        if self == &dest {
            return Vec::new();
        }
        // if we're in a hallway...
        if let Position::Hallway(i) = self {
            // ...and the destination is a hallway, take the
            // positions that lie between and put them in a list
            if let Position::Hallway(j) = dest {
                let mut positions = Vec::new();
                // account for moving to the left
                if j < *i {
                    for k in (j..*i).rev() {
                        positions.push(Position::Hallway(k));
                    }
                } else {
                    for k in (*i..=j).skip(1) {
                        positions.push(Position::Hallway(k));
                    }
                }
                return positions;
            }
            // ...and the destination is a burrow, add the positions
            // up until the hallway above the burrow...
            if let Position::Burrow(i, j) = dest {
                let mut positions = self.path(dest.hallway_above().unwrap());
                // if dest is second burrow pos, add first burrow pos
                if j == 2 {
                    positions.push(Position::Burrow(i, 1));
                }
                // add dest
                positions.push(dest);
                return positions;
            }
        }
        // if we're in a burrow...
        if let Position::Burrow(i, l) = self {
            // ...and the destination is a burrow...
            if let Position::Burrow(k, _) = dest {
                // ...which is the same burrow, return a list with only the destination
                if *i == k {
                    return vec![dest];
                }
                // if it's a different burrow, first get the path to the hallway above
                let mut positions = self.path(dest.hallway_above().unwrap());
                // then add the path from the hallway above, to the burrow
                positions.extend(dest.hallway_above().unwrap().path(dest));
                return positions;
            }
            // ...and the destination is a hallway, add the positions needed to get to the hallway
            if let Position::Hallway(_) = dest {
                let mut positions = vec![];
                // if self is second burrow pos, add first burrow pos
                if *l == 2 {
                    positions.push(Position::Burrow(*i, 1));
                }
                // add the hallway above the burrow
                positions.push(self.hallway_above().unwrap());
                // add the positions from the hallway above to the destination
                positions.extend(self.hallway_above().unwrap().path(dest));
                return positions;
            }
        }
        panic!("Can't find path from {:?} to {:?}", self, dest);
    }

    /// Return the hallway position above a burrow number
    fn hallway_above(&self) -> Option<Position> {
        if let Position::Burrow(i, _) = self {
            Some(Position::Hallway(*i * 2))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Amphipod {
    position: Position,
    r#type: Type,
    left_home: bool,
}

impl Amphipod {
    fn new(position: Position, r#type: Type) -> Self {
        Amphipod {
            position,
            r#type,
            left_home: false,
        }
    }

    fn move_to(&mut self, position: Position) {
        self.position = position;
        // if the new position is not our home, we have left home.
        if !self.is_home() {
            self.left_home = true;
        }
    }

    fn is_home(&self) -> bool {
        match self.position {
            Position::Hallway(_) => false,
            Position::Burrow(nr, _) => nr == self.r#type.dest_burrow(),
        }
    }

    fn left_home(&self) -> bool {
        self.left_home
    }

    fn is_in_burrow(&self, nr: usize) -> bool {
        match self.position {
            Position::Hallway(_) => false,
            Position::Burrow(i, _) => i == nr,
        }
    }

    fn is_in_bottom(&self) -> bool {
        match self.position {
            Position::Hallway(_) => false,
            Position::Burrow(_, i) => i == BURROW_DEPTH,
        }
    }
}

/// A struct that holds the map and the spent energy, so we can pass it around
/// in a backtracking context if we need to.
#[derive(Debug, Clone, Eq)]
struct Map {
    pods: Vec<Amphipod>,
    energy: usize,
}

// implement EQ for Map so we can use it in a HashSet
impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.str_rep() == other.str_rep()
        // self.pods == other.pods 
        && self.energy == other.energy
    }
}

// implement hash for map so we can use it in a HashSet
impl std::hash::Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.str_rep().hash(state);
        self.energy.hash(state);
    }
}

type Move = (Amphipod, Position);

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut map = vec![vec![' '; HALLWAY_SIZE]; BURROW_DEPTH + 1];
        // mark the spaces where the pods can go
        (0..HALLWAY_SIZE).for_each(|i| {
            map[0][i] = '.';
        });
        (1..=BURROW_DEPTH).for_each(|i| {
            map[i][2] = '.';
            map[i][4] = '.';
            map[i][6] = '.';
            map[i][8] = '.';
        });
        for pod in &self.pods {
            match pod.position {
                Position::Hallway(i) => {
                    map[0][i] = pod.r#type.to_char();
                }
                Position::Burrow(i, j) => {
                    map[j][i * 2] = pod.r#type.to_char();
                }
            }
        }
        for row in map.iter() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    /// Tests if the map is complete
    /// Every amphipod is in its burrow
    fn complete(&self) -> bool {
        self.pods.iter().all(|a| a.is_home())
    }

    fn str_rep(&self) -> String {
        format!("{}", self)
    }

    fn possible_moves(&self) -> Vec<Move> {
        // for every amphipod
        let mut moves = Vec::new();
        self.pods.iter().for_each(|pod| {
            // any pod that is in the bottom burrow and is home should not move
            if pod.is_in_bottom() && pod.is_home() {
                return;
            }

            // any pod that left home, and is now 'home' should not move anymore.
            if pod.left_home() && pod.is_home() {
                return;
            }

            // for each free position, get the path to it from this pod
            self.free_spaces().iter().for_each(|f| {
                // if f is a burrow
                if let Position::Burrow(i, _) = f {
                    // and the pod is in the hallway
                    // if let Position::Hallway(_) = pod.position {

                    // and the pod is not in this burrow already
                    if !pod.is_in_burrow(*i) {
                        // we should skip this burrow if it is not the destination of the pod
                        if *i != pod.r#type.dest_burrow() {
                            return;
                        }
                        // we're looking at the destination burrow, but we should skip the move, if
                        // the burrow already contains an amphipod with a type that does not have the burrow
                        // as destionation burrow
                        if self.pods.iter().any(|op| {
                            if let Position::Burrow(oi, _) = op.position {
                                return (oi == *i) && (op.r#type.dest_burrow() != *i);
                            }
                            false
                        }) {
                            return;
                        }
                    } else {
                        // if we're already in this burrow, we should skip this move
                        return;
                    }
                }

                // if f is a hallway
                if let Position::Hallway(_) = f {
                    // and the pod is in a hallway
                    if let Position::Hallway(_) = pod.position {
                        // skip this move, pods in a hallway can only move into burrows
                        return;
                    }
                }

                // get the path from the pod to this free space
                let path = pod.position.path(*f);
                // if all the positions in the path are not occupied
                if path.iter().all(|p| !self.occupied(*p)) {
                    // add the move to the list of moves
                    moves.push((*pod, *f));
                }
            });
        });
        moves
            .into_iter()
            .filter(|(_, pos)| {
                if let Position::Hallway(i) = pos {
                    *i != 2 && *i != 4 && *i != 6 && *i != 8
                } else {
                    true
                }
            })
            .collect()
    }

    fn occupied(&self, pos: Position) -> bool {
        self.pods.iter().any(|a| a.position == pos)
    }

    fn free_spaces(&self) -> Vec<Position> {
        Position::all()
            .into_iter()
            .filter(|p| !self.occupied(*p))
            .collect()
    }

    /// Do a move,
    fn do_move(&self, m: &Move) -> Self {
        let (pod, pos) = *m;

        // find the number of spaces between the amphipod pos
        // and the pos of the move.
        let distance = pod.position.path(pos).len();

        let mut new = self.clone();

        new.energy += distance * pod.r#type.energy_cost();

        // find the pod in our list of pods, so we can update its position
        new.pods
            .iter_mut()
            .find(|p| p.position == pod.position)
            .unwrap()
            .move_to(pos);

        new
    }

    /// Get the best solution of the map
    /// This is the solution with the lowest energy cost
    fn best_solution(&self) -> Option<Self> {
        println!("{}", self);

        // if the map is already complete, return it
        if self.complete() {
            // println!("E: {:?} - Map is complete", self.energy);
            return Some(self.clone());
        }

        // get the possible moves
        let moves = self.possible_moves();

        // println!("E: {:?} - Possible moves: {:?}:", self.energy, moves.len());
        // printout all the moves:
        // moves.iter().for_each(|m| {
        //     println!("             {:?}", m);
        // });

        // if there are no moves, return None
        if moves.is_empty() {
            return None;
        }

        // for each move, do the move and get the new map
        // get all the solutions of the new map
        // find the solution with the lowest energy cost
        moves
            .into_iter()
            .filter_map(|m| {
                // println!("E: {:?} - Doing move: {:?}", self.energy, m);
                let new = self.do_move(&m);
                new.best_solution()
            })
            .min_by(|a, b| a.energy.cmp(&b.energy))
    }

    /// The recursive approach may not be the best way to solve this problem. A different approach would be to use
    /// some kind of stack hashmap, a visited hashmap, and a solution hashmap. The stack would become filled with the
    /// maps we need to solve, and the visited hashmap should prevent us from visiting the same map twice.
    /// The solution hashmap would contain the best solution for each map.
    fn best_solution_imperative(&self) -> Option<Self> {
        // if the map is already complete, return it
        if self.complete() {
            return Some(self.clone());
        }

        // get the possible moves
        let moves = self.possible_moves();

        // if there are no moves, return None
        if moves.is_empty() {
            return None;
        }

        // create a stack hashmap
        let mut stack: VecDeque<Map> = VecDeque::new();

        // create a visited hashset
        let mut visited = HashSet::<Map>::new();

        // create a solution hashset
        let mut solutions = HashSet::<Map>::new();

        // keep track of lowest energy
        let mut lowest_energy = usize::max_value();

        // add the current map to the stack
        stack.push_back(self.clone());

        // while the stack is not empty
        while !stack.is_empty() {
            // get the first map in the stack
            let map = stack.pop_front().unwrap();

            // if the energy of this map is already higher than the lowest seen, skip it
            if map.energy > lowest_energy {
                continue;
            }

            // if the map is already visited, skip it
            if visited.contains(&map) {
                continue;
            }

            // printout stack size, visited size, and solutions size
            println!(
                "EN:{:>10} ST:{:>10} VI:{:>10} SO:{:>10} LW:{:>10}",
                map.energy,
                stack.len(),
                visited.len(),
                solutions.len(),
                lowest_energy
            );
            // print!("{}", map);

            // add the map to the visited hashset
            visited.insert(map.clone());

            // if the map is complete, add it to the solutions hashset
            if map.complete() {
                // println!("  -- Found solution, E: {}", map.energy);
                solutions.insert(map.clone());
                // update lowest energy if this energy is lower
                if map.energy < lowest_energy {
                    lowest_energy = map.energy;
                }
            }

            // get the possible moves for the map
            let moves = map.possible_moves();
            // println!("  -- Fonud {:?} new moves", moves.len());

            // for each move, do the move and get the new map
            // add the new map to the stack
            moves.into_iter().for_each(|m| {
                let new = map.do_move(&m);

                // if the new map has higher energy than the lowest seen, skip it
                if new.energy > lowest_energy {
                    return;
                }

                // if the new map is already visited, skip it
                if visited.contains(&new) {
                    return;
                }

                // if the map is already in the stack, skip it
                if stack.contains(&new) {
                    return;
                }

                // if the map is already a solution, skip it
                if solutions.contains(&new) {
                    return;
                }

                //stack.push_back(new);
                stack.push_front(new);
                // instead of pushing the map to the end or the front of the stack,
                // find the place based on the energy of the map, and insert it there.
                //let idx = stack.partition_point(|m| m.energy < new.energy);
                //stack.insert(idx, new);
            });

            // sort the stack by energy
        }

        // find the solution with the lowest energy cost
        solutions.into_iter().min_by(|a, b| a.energy.cmp(&b.energy))
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let mut pods = Vec::new();
        (1..=4).for_each(|b_nr| {
            (1..=2).for_each(|bpos| {
                let c = lines[1 + bpos].chars().nth(b_nr * 2 + 1).unwrap();
                pods.push(match c {
                    'A' => Amphipod::new(Position::Burrow(b_nr, bpos), r#Type::A),
                    'B' => Amphipod::new(Position::Burrow(b_nr, bpos), r#Type::B),
                    'C' => Amphipod::new(Position::Burrow(b_nr, bpos), r#Type::C),
                    'D' => Amphipod::new(Position::Burrow(b_nr, bpos), r#Type::D),
                    _ => panic!("Invalid Amphipod: {}", c),
                });
            });
        });
        Ok(Map { pods, energy: 0 })
    }
}
pub fn main() {
    let map = Map::from_str(
        r#"#############
#...........#
###B#C#A#D###
  #B#C#D#A#
  #########"#,
    )
    .unwrap();

    let solution = map.best_solution_imperative().unwrap();

    println!("{} e:{}", solution, solution.energy);
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_read_map() {
        let map = Map::from_str(
            r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#,
        )
        .unwrap();

        // the map should have 8 amphipods
        assert_eq!(map.pods.len(), 8);

        assert_eq!(map.pods[0].r#type, Type::B);
        assert_eq!(map.pods[0].position, Position::Burrow(1, 1));
        assert_eq!(map.pods[1].r#type, Type::A);
        assert_eq!(map.pods[1].position, Position::Burrow(1, 2));

        assert_eq!(map.pods[2].r#type, Type::C);
        assert_eq!(map.pods[2].position, Position::Burrow(2, 1));
        assert_eq!(map.pods[3].r#type, Type::D);
        assert_eq!(map.pods[3].position, Position::Burrow(2, 2));

        assert_eq!(map.pods[4].r#type, Type::B);
        assert_eq!(map.pods[4].position, Position::Burrow(3, 1));
        assert_eq!(map.pods[5].r#type, Type::C);
        assert_eq!(map.pods[5].position, Position::Burrow(3, 2));

        assert_eq!(map.pods[6].r#type, Type::D);
        assert_eq!(map.pods[6].position, Position::Burrow(4, 1));
        assert_eq!(map.pods[7].r#type, Type::A);
        assert_eq!(map.pods[7].position, Position::Burrow(4, 2));
    }

    #[test]
    fn test_map_complete() {
        let pods = vec![
            Amphipod::new(Position::Burrow(1, 1), Type::B),
            Amphipod::new(Position::Burrow(1, 2), Type::C),
            Amphipod::new(Position::Burrow(2, 1), Type::D),
            Amphipod::new(Position::Burrow(2, 2), Type::A),
            Amphipod::new(Position::Burrow(3, 1), Type::B),
            Amphipod::new(Position::Burrow(3, 2), Type::A),
            Amphipod::new(Position::Burrow(4, 1), Type::C),
            Amphipod::new(Position::Burrow(4, 2), Type::D),
        ];
        let map = Map { pods, energy: 0 };

        // this map should not be complete
        assert!(!map.complete());

        // create a map that is complete
        let pods = vec![
            Amphipod::new(Position::Burrow(1, 1), Type::A),
            Amphipod::new(Position::Burrow(1, 2), Type::A),
            Amphipod::new(Position::Burrow(2, 1), Type::B),
            Amphipod::new(Position::Burrow(2, 2), Type::B),
            Amphipod::new(Position::Burrow(3, 1), Type::C),
            Amphipod::new(Position::Burrow(3, 2), Type::C),
            Amphipod::new(Position::Burrow(4, 1), Type::D),
            Amphipod::new(Position::Burrow(4, 2), Type::D),
        ];

        let map = Map { pods, energy: 0 };

        // this map should be complete
        assert!(map.complete());
    }

    #[test]
    fn test_path() {
        let pos = Position::Burrow(1, 1);
        let path = pos.path(Position::Burrow(1, 2));
        // assert that path contains only Position::Burrow(1, 2)
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], Position::Burrow(1, 2));

        let path = pos.path(Position::Burrow(2, 1));
        let expected = vec![
            Position::Hallway(2),
            Position::Hallway(3),
            Position::Hallway(4),
            Position::Burrow(2, 1),
        ];
        assert_eq!(path, expected);

        let path = pos.path(Position::Hallway(10));
        let expected = vec![
            Position::Hallway(2),
            Position::Hallway(3),
            Position::Hallway(4),
            Position::Hallway(5),
            Position::Hallway(6),
            Position::Hallway(7),
            Position::Hallway(8),
            Position::Hallway(9),
            Position::Hallway(10),
        ];
        assert_eq!(path, expected);

        let path = pos.path(Position::Hallway(0));
        let expected = vec![
            Position::Hallway(2),
            Position::Hallway(1),
            Position::Hallway(0),
        ];
        assert_eq!(path, expected);
    }

    #[test]
    fn test_possible_moves() {
        let pods = vec![
            Amphipod::new(Position::Burrow(1, 2), Type::B),
            Amphipod::new(Position::Hallway(3), Type::D),
        ];
        let map = Map { pods, energy: 0 };

        let moves = map.possible_moves();

        // print the moves line by line
        for (p, m) in moves.iter() {
            println!("Move: {:?} to {:?}", p, m);
        }

        assert_eq!(moves.len(), 4);

        // the moves should contain no moves to the spaces right above the burrows
        assert!(
            moves.iter().all(|(_, m)| {
                if let Position::Hallway(i) = m {
                    *i != 2 && *i != 4 && *i != 6 && *i != 8
                } else {
                    true
                }
            }),
            "Moves should not contain moves to the spaces right above the burrows"
        );

        // Amphipods will never move from the hallway into a room unless that room is
        // their destination room and that room contains no amphipods which do not also
        // have that room as their own destination.
        assert!(
            moves.iter().all(|(p, m)| {
                if let Position::Hallway(_) = p.position {
                    if let Position::Burrow(i, _) = m {
                        return *i == p.r#type.dest_burrow();
                    }
                }
                true
            }),
            "Amphipods will never move from the hallway into a room unless that room is their destination room"
        );

        // Amphipods that are in the hallway, will only be able to move into a room.
        // There should be no moves for type D into a Hallway position
        assert!(
            moves.iter().all(|(p, m)| {
                if let Position::Hallway(_) = p.position {
                    if let Position::Hallway(_) = m {
                        return false;
                    }
                }
                true
            }),
            "Amphipods that are in the hallway, will only be able to move into a room"
        );

        // Move the first amphipod in the last burrow. The second amphipod should be able to move into his burrow
        let pods = vec![
            Amphipod::new(Position::Burrow(4, 2), Type::B),
            Amphipod::new(Position::Hallway(3), Type::D),
        ];

        let map = Map { pods, energy: 0 };

        let moves = map.possible_moves();

        // there should be no moves with the D type amphipod into the fourth burrow
        assert!(
            moves.iter().all(|(p, m)| {
                if let Type::D = p.r#type {
                    if let Position::Burrow(i, _) = m {
                        return *i != 4;
                    }
                }
                true
            }),
            "The D type amphipod should not be able to move into the 
            fourth burrow, if it contains no other D type amphipods"
        );

        // Put a D amphipod in the last burrow, it's home and there should be no moves
        let pod = Amphipod::new(Position::Burrow(4, 2), Type::D);
        let pods = vec![pod];

        let map = Map { pods, energy: 0 };

        // move the pod out into the hallway and back in again.
        let pod = map.pods[0];
        let map = map.do_move(&(dbg!(pod), Position::Hallway(3)));
        let pod = map.pods[0];
        let map = map.do_move(&(dbg!(pod), Position::Burrow(4, 2)));

        // it should now have 'left home' and there should be no moves

        let moves = map.possible_moves();

        // moves should be empty
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_map_equality() {
        // two maps with the same pods but in a different order, should be the same
        let pods1 = vec![
            Amphipod::new(Position::Burrow(1, 1), Type::A),
            Amphipod::new(Position::Burrow(2, 1), Type::B),
        ];
        let map1 = Map { pods: pods1, energy: 0 };

        let pods2 = vec![
            Amphipod::new(Position::Burrow(2, 1), Type::B),
            Amphipod::new(Position::Burrow(1, 1), Type::A),
        ];
        let map2 = Map { pods: pods2, energy: 0 };

        // map1 and map2 should be equal.
        assert_eq!(map1, map2);
    }

    #[test]
    fn test_move_energy_single_a() {
        // start with a simple map with an amphipod in the first burrow
        let pods = vec![Amphipod::new(Position::Burrow(1, 1), Type::A)];

        let map = Map { pods, energy: 0 };

        // create a move from the first burrow to the second burrow
        let m = (map.pods[0], Position::Burrow(1, 2));
        // perform the move
        let map = map.do_move(&m);

        // pod should have moved 1 space, and as a type A, it should have consumed 1 energy
        assert_eq!(map.energy, 1);
    }

    #[test]
    fn test_move_energy_single_b() {
        // start with a simple map with an amphipod in the first burrow
        let pods = vec![Amphipod::new(Position::Burrow(1, 1), Type::B)];

        let map = Map { pods, energy: 0 };

        // create a move from the first burrow to the hallway
        let m = (map.pods[0], Position::Hallway(3));
        // perform the move
        let map = map.do_move(&m);

        // pod should have moved 2 spaces, and as a type B, it should have consumed 20 energy
        assert_eq!(map.energy, 20);
    }

    #[test]
    fn test_move_energy_double_c_d() {
        // start with a simple map with an amphipod in the first burrow
        let pods = vec![
            Amphipod::new(Position::Burrow(1, 1), Type::C),
            Amphipod::new(Position::Burrow(3, 1), Type::D),
        ];

        let map = Map { pods, energy: 0 };

        // move the c amphipod to the left of the hallway, 3 moves
        let map = map.do_move(&(map.pods[0], Position::Hallway(0)));

        // map energy should be 300
        assert_eq!(map.energy, 300);

        // move the d amphipod to the right of the hallway, 5 moves
        let map = map.do_move(&(map.pods[1], Position::Hallway(10)));

        // 3x100 + 5x1000 = 5300 energy
        assert_eq!(map.energy, 5300);
    }

    // #[test]
    fn test_solving_single() {
        // create a map with a Type B amphipod in the first burrow
        let pods = vec![Amphipod::new(Position::Burrow(1, 1), Type::B)];

        let map = Map { pods, energy: 0 };

        // The solution of the map should be a single move from the first burrow
        // to the second burrow.
        let solution = map.best_solution_imperative().unwrap();

        // the solution should contain only one B type amphipod
        assert_eq!(solution.pods.len(), 1);
        // that ends up in the bottom position of the second burrow.
        assert_eq!(solution.pods[0].position, Position::Burrow(2, 1));

        // the solution should consume 40 energy
        assert_eq!(solution.energy, 40);

        println!("Solution:\n{}\nE: {}", solution, solution.energy);
    }

    #[test]
    fn test_solving_double() {
        // now test with two amphipods.
        // Before the B amphipod can go to it's burrow,
        // The D amphipod needs to move along.
        let pods = vec![
            Amphipod::new(Position::Burrow(1, 2), Type::B),
            Amphipod::new(Position::Hallway(3), Type::D),
        ];
        let map = Map { pods, energy: 0 };

        let solution = map.best_solution_imperative().unwrap();

        // assuming the D amphipod can move to the fourth burrow,
        // it will move to Position::Burrow(4, 1)
        assert_eq!(solution.pods[1].position, Position::Burrow(4, 1));

        // assuming the B amphipod can now move to the second burrow,
        // it will move to Position::Burrow(2, 1)
        assert_eq!(solution.pods[0].position, Position::Burrow(2, 1));

        // D should have moved 6 spaces for a total of 6000 energy
        // B should have moved 5 spaces for a total of 40 energy
        assert_eq!(solution.energy, 6000 + 50);

        println!("Solution:\n{}\nE: {}", solution, solution.energy);
    }

    #[test]
    fn test_solving_example() {
        let map = Map::from_str(
            r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#,
        )
        .unwrap();

        let solution = map.best_solution_imperative().unwrap();

        // the solution should have took 12521 energy
        assert_eq!(solution.energy, 12521);
    }
}
