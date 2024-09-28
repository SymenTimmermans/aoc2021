use std::str::FromStr;

/// --- Day 25: Sea Cucumber ---
/// We have moving sea cucumbers, either moving south or east. We need to find the number of moves
/// it takes until all sea cucumbers stop moving.
/// I don't have a clue how this is best implemented.
///
/// Maybe first choose a representation of a tile in the map
type Tile = u8;
const EMPTY: Tile = 0;
const EAST: Tile = 1;
const SOUTH: Tile = 2;

/// Most basic implementation of a map
#[derive(Debug, PartialEq)]
struct Map {
    pub size: (usize, usize),
    pub tiles: Vec<Tile>,
}

/// Parser
impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut tiles = Vec::new();
        let mut size = (0, 0);
        for line in s.lines() {
            size.1 = line.len();
            for c in line.chars() {
                match c {
                    '.' => tiles.push(EMPTY),
                    '>' => tiles.push(EAST),
                    'v' => tiles.push(SOUTH),
                    _ => panic!("Invalid character in map"),
                }
            }
            size.0 += 1;
        }
        Ok(Map { size, tiles })
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                let c = match self.tiles[i * self.size.1 + j] {
                    EMPTY => '.',
                    EAST => '>',
                    SOUTH => 'v',
                    _ => panic!("Invalid tile"),
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

/// Implementation
impl Map {
    fn has_moves(&self, indices: &[usize], t: Tile) -> bool {
        let len = indices.len();
        for i in 0..len - 1 {
            let current = self.tiles[indices[i]];
            let next = self.tiles[indices[i + 1]];

            if current == t && next == EMPTY {
                return true;
            }
        }
        false
    }

    fn col_stopped(&self, y: usize) -> bool {
        let indices = self.col_indices(y);
        !self.has_moves(&indices, SOUTH)
    }

    fn row_stopped(&self, x: usize) -> bool {
        let indices = self.row_indices(x);
        !self.has_moves(&indices, EAST)
    }

    fn stopped(&self) -> bool {
        for i in 0..self.size.0 {
            if !self.row_stopped(i) {
                return false;
            }
        }
        for i in 0..self.size.1 {
            if !self.col_stopped(i) {
                return false;
            }
        }
        true
    }

    fn move_cucumbers(&mut self, indices: &[usize], direction: Tile) {
        let len = indices.len();
        if len < 2 {
            return;  // Nothing to do if we have fewer than 2 indices
        }

        let mut swaps: Vec<(usize, usize)> = Vec::new();

        let mut i = 0;
        while i < len - 1 {  // -1 because the last index is the same as the first
            let current = indices[i];
            let next = indices[i + 1];
            
            if self.tiles[current] == direction && self.tiles[next] == EMPTY {
                swaps.push((current, next));
                i += 2;  // Skip the next position as we've just queued a move there
            } else {
                i += 1;
            }
        }

        // Now perform all the swaps
        for (from, to) in swaps {
            self.tiles.swap(from, to);
        }
    }

    // Updated helper functions to include the first index at the end
    fn row_indices(&self, x: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.size.1).map(|y| x * self.size.1 + y).collect();
        indices.push(indices[0]); // Add the first index to the end
        indices
    }

    fn col_indices(&self, y: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.size.0).map(|x| x * self.size.1 + y).collect();
        indices.push(indices[0]); // Add the first index to the end
        indices
    }

    // Updated mov_row and mov_col functions
    fn mov_row(&mut self, x: usize) {
        let indices = self.row_indices(x);
        self.move_cucumbers(&indices, EAST);
    }

    fn mov_col(&mut self, y: usize) {
        let indices = self.col_indices(y);
        self.move_cucumbers(&indices, SOUTH);
    }

    fn mov(&mut self) {
        for i in 0..self.size.0 {
            self.mov_row(i);
        }
        for i in 0..self.size.1 {
            self.mov_col(i);
        }
    }

    fn solve(&mut self) -> i32 {
        let mut moves = 0;
        while !self.stopped() {
            self.mov();
            moves += 1;
        }
        moves + 1
    }
}

// ----------------
pub fn main() {
    let input = include_str!("../../input/day25.txt");
    let mut map = Map::from_str(input).unwrap();
    let moves = map.solve();
    println!("Number of moves until all cucumbers stopped: {}", moves);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_from_str() {
        let map = Map::from_str(r#"..>v."#).unwrap();
        let map_exp = Map {
            size: (1, 5),
            tiles: vec![EMPTY, EMPTY, EAST, SOUTH, EMPTY],
        };
        assert_eq!(map, map_exp);
    }

    #[test]
    fn test_map_display() {
        let map = Map::from_str(r#"..>v."#).unwrap();
        let map_str = format!("{}", map);
        assert_eq!(
            map_str,
            r#"..>v.
"#
        );
    }

    #[test]
    fn test_row_stopped() {
        let map = Map::from_str(r#"..>.."#).unwrap();
        assert!(!map.row_stopped(0));

        let map = Map::from_str(r#"..>v."#).unwrap();
        assert!(
            map.row_stopped(0),
            "row should stop if a south cucumber blocks an east one"
        );

        let map = Map::from_str(r#"..>v>"#).unwrap();
        assert!(!map.row_stopped(0), "row is not stopped if it can wrap");
    }

    #[test]
    fn test_col_stopped() {
        let map = Map::from_str(
            r#".....
..v..
....."#,
        )
        .unwrap();
        assert!(!map.col_stopped(2));

        let map = Map::from_str(
            r#".....
..v..
..>.."#,
        )
        .unwrap();
        assert!(
            map.col_stopped(2),
            "col should stop if an east cucumber blocks a south one"
        );
        let map = Map::from_str(
            r#".....
.....
..v.."#,
        )
        .unwrap();
        assert!(!map.col_stopped(2), "col is not stopped if it can wrap");
    }

    #[test]
    fn test_map_stopped() {
        let map = Map::from_str(
            r#"..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v.."#,
        )
        .unwrap();
        assert!(map.stopped());
    }

    #[test]
    fn test_row_indexes() {
        let map = Map::from_str(r#"..>v."#).unwrap();
        assert_eq!(map.row_indices(0), vec![0, 1, 2, 3, 4, 0]);
    }

    #[test]
    fn test_col_indexes() {
        let map = Map::from_str(
            r#"..>v.
.....
....."#,
        )
        .unwrap();
        assert_eq!(map.col_indices(2), vec![2, 7, 12, 2]);
    }

    #[test]
    fn test_map_move1() {
        let mut map = Map::from_str(r#"..>.."#).unwrap();
        let map_exp = Map::from_str(r#"...>."#).unwrap();

        map.mov_row(0);
        assert_eq!(map, map_exp);
    }

    #[test]
    fn test_map_move2() {
        let mut map = Map::from_str(r#"....>"#).unwrap();
        let map_exp = Map::from_str(r#">...."#).unwrap();

        map.mov_row(0);
        assert_eq!(map, map_exp);
    }

    #[test]
    fn test_example() {
        let input = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;
        let mut map = Map::from_str(input).unwrap();
        let moves = map.solve();
        assert_eq!(moves, 58);
    }

    #[test]
    fn test_example2() {
        let input = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

        let cases = vec![(1, r#"....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"#),
            (2, r#">.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv."#)
        ];
        for (moves, state) in cases {
            let mut map = Map::from_str(input).unwrap();
            // do the moves
            for _ in 0..moves {
                map.mov();
            }
            let expected = Map::from_str(state).unwrap();
            assert_eq!(expected, map, "after {} moves,\n{}\nshould be\n{}", moves, map, expected);
        }
    }


    #[test]
    fn test_last_steps_example() {
        let input = r#"..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v.."#;
        let mut map = Map::from_str(input).unwrap();
        let moves = map.solve();
        assert_eq!(moves, 1);
    }

}
