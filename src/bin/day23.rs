use cached::proc_macro::cached;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

/// We need an efficient way to represent the map map in order to use efficient ways of checking
/// possible moves. We have 11 hallway positions, but 4 of them are unusable, so that leaves 7
/// possible positions.
/// We also need to represent the burrows, so we can have a map that sort of looks like this:
///
/// .. . . . .. ABCDABCD
/// Compressed:
/// .......ABCDABCD
///
/// We can use a string for this
type Map = String;

/// These are the desired win maps.
const WIN_MAP: &str = ".......ABCDABCD";
const WIN_MAP_XL: &str = ".......ABCDABCDABCDABCD";

/// a pod is simply a char in the map
type Pod = (usize, char);

/// a route is a series of indexes not including the start index
type Route = Vec<usize>;

/// Return an iterator over the pods in the map
fn pods_iter(map: &Map) -> impl Iterator<Item = Pod> + '_ {
    map.chars()
        .enumerate()
        .filter(|(_, c)| ['A', 'B', 'C', 'D'].contains(c))
}

/// Return true if the pod is in a burrow
fn in_burrow(pod: &Pod) -> bool {
    pod.0 >= 7
}

/// Return true if the pod is in it's home burrow
fn in_home_burrow(pod: &Pod) -> bool {
    if !in_burrow(pod) {
        return false;
    }

    match (pod.0 - 7) % 4 {
        0 => pod.1 == 'A',
        1 => pod.1 == 'B',
        2 => pod.1 == 'C',
        3 => pod.1 == 'D',
        _ => false,
    }
}

/// Return true if the pod is in the hallway
fn in_hallway(pod: &Pod) -> bool {
    pod.0 < 7
}

/// Return the energy cost for the movement of a pod
fn energy(pod: &Pod) -> u32 {
    match pod.1 {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

/// Return the contents of the burrow. For simplicity sake, we can pass in a Char, since
/// that's likely what we have from the context that we make this call.
fn burrow(c: &char, map: &Map) -> Vec<char> {
    // only return the chars that are in the burrow
    match c {
        'A' => map.chars().skip(7).step_by(4).collect(),
        'B' => map.chars().skip(8).step_by(4).collect(),
        'C' => map.chars().skip(9).step_by(4).collect(),
        'D' => map.chars().skip(10).step_by(4).collect(),
        _ => vec![],
    }
}

/// Check if the pod may move, by looking at some easy facts like it being in a hallway
/// or in a burrow. If it's in a burrow, it may only move if the burrow contains other
/// characters than the pod.
fn may_move(pod: &Pod, map: &Map) -> bool {
    if !in_burrow(pod) {
        return true;
    }

    if !in_home_burrow(pod) {
        return true;
    }

    // if the pod is in it's destination burrow, and the burrow contains no other pods, it's not
    // allowed to move
    let (_, c) = pod;
    let burrow = burrow(c, map);

    // burrow should contain other characters than c for the pod to be allowed to move
    burrow.iter().filter(|x| *x != c && *x != &'.').count() > 0
}

/// Return a route from one position to another. This is a bit tricky, since we have to take
/// into account that we can move from the hallway to the burrow and vice versa. The route
/// is a vector of indexes, where the last index is the destination index.
fn trace(from: usize, to: usize) -> Route {
    let mut r = vec![];
    if to >= 7 {
        // going to a burrow, but which one
        let mut b = (to - 7) % 4 + 1;
        let mut drop_delta = 6;
        // lets go to the top of the burrow
        // 01 2 3 4 56
        //   1 2 3 4
        // first see if we need to go left or right
        let dir: i8 = if from <= b { 1 } else { -1 };
        // if we move to the left, we need to add 1 to the burrow index, because we can drop down
        // a bit earlier, ie 6 > 5 > burrow D instead of 3 > 4 > burrow D
        // we also adjust the drop delta for this case
        if dir == -1 {
            b += 1;
            drop_delta = 5;
        }
        let mut pos = from;
        while pos != b {
            pos = (pos as i8 + dir) as usize;
            r.push(pos);
        }
        // now let's go down the burrow
        pos += drop_delta;
        r.push(pos);
        while pos != to {
            pos += 4;
            r.push(pos);
        }
    } else {
        // going to the hallway
        let mut pos = from;
        while pos >= 11 {
            pos -= 4;
            r.push(pos);
        }
        // now we are on the edge of the hallway
        // subtract 6 to get to the top of the hallway
        pos -= 6;
        // we default to the pos to the left of the top, but if we need to go right,
        // we need to add one
        if pos < to {
            pos += 1;
        }
        r.push(pos);
        while pos != to {
            if pos < to {
                pos += 1;
            } else {
                pos -= 1;
            }
            r.push(pos);
        }
    }
    r
}

/// Return the number of steps the pod has to take, taking into account the positions we don't
/// consider a valid end destination, like right outside the burrow.
/// It means we have to take into account the steps it takes to move into the burrow and out of it.
/// Plus also whether we cross a burrow entrance.
fn route_steps(route: &Route) -> u32 {
    // take the route without the last item
    let path = &route[..route.len() - 1];
    // count how many of indexes 2, 3, 4, are in the path
    let skipped = path
        .iter()
        .filter(|i| **i >= 2_usize && **i <= 4_usize)
        .count() as u32;
    // add the skipped count plus one for moving in and out of a burrow
    route.len() as u32 + skipped + 1
}

/// Return a vector of possible routes a pod can take, without considering if those positions are
/// occupied or not, that will be handled by a different function.
fn routes_from(pod: &Pod, deep: bool) -> Vec<Route> {
    let mut routes = vec![];
    if in_burrow(pod) {
        // we can move to the hallway
        for i in 0..7 {
            routes.push(trace(pod.0, i));
        }
    } else {
        // in the hallway, can only move back into home burrow
        match pod.1 {
            'A' => {
                routes.push(trace(pod.0, 7));
                routes.push(trace(pod.0, 11));
            }
            'B' => {
                routes.push(trace(pod.0, 8));
                routes.push(trace(pod.0, 12));
            }
            'C' => {
                routes.push(trace(pod.0, 9));
                routes.push(trace(pod.0, 13));
            }
            'D' => {
                routes.push(trace(pod.0, 10));
                routes.push(trace(pod.0, 14));
            }
            _ => {}
        }
        if deep {
            match pod.1 {
                'A' => {
                    routes.push(trace(pod.0, 15));
                    routes.push(trace(pod.0, 19));
                }
                'B' => {
                    routes.push(trace(pod.0, 16));
                    routes.push(trace(pod.0, 20));
                }
                'C' => {
                    routes.push(trace(pod.0, 17));
                    routes.push(trace(pod.0, 21));
                }
                'D' => {
                    routes.push(trace(pod.0, 18));
                    routes.push(trace(pod.0, 22));
                }
                _ => {}
            }
        }
    }
    // remove the routes that are empty
    routes.iter().filter(|r| !r.is_empty()).cloned().collect()
}

/// Check the map to see if a route is clear of any other pods.
pub fn route_clear(route: &Route, map: &Map) -> bool {
    route.iter().all(|i| map.chars().nth(*i).unwrap() == '.')
}

/// Convenience function to print the map in the same format as is used in the puzzle.
pub fn print_map(map: &Map) {
    // print the top row
    println!("#############");
    print!("#");
    for i in 0..7 {
        let c = map.chars().nth(i).unwrap();
        print!("{}", c);
        // if i in (1, 2, 3, 4), print an extra .
        // to take into account burrow entrances
        if (1..=4).contains(&i) {
            print!(".");
        }
    }
    println!("#");
    let mut i = 7;
    while i < map.len() {
        if i == 7 {
            print!("###");
        } else {
            print!("  #")
        };
        for j in 0..4 {
            let c = map.chars().nth(i + j).unwrap();
            print!("{}#", c);
        }
        if i == 7 {
            println!("##");
        } else {
            println!("  ")
        };
        i += 4;
    }
    println!("  #########  ");
}

/// Now we need a function that returns the lowest empty burrows, so we can filter out the routes
/// that have a destination burrow position that is not the lowest.
#[cached(key = "String", convert = r#"{ String::from(map) }"#)]
fn get_lowest_empty_burrows(map: &Map) -> Vec<usize> {
    let mut lowest = [0_usize; 4];
    let mut i = 7;
    while i < map.len() {
        for j in 0..4 {
            if map.chars().nth(i + j).unwrap() == '.' {
                lowest[j] = i + j;
            }
        }
        i += 4;
    }
    // return a vector of lowest, filtering out the ones that are 0
    lowest.iter().filter(|x| **x != 0).cloned().collect()
}

/// Check if the burrow is dirty, meaning that it contains other pods than the pod that is moving
/// into it.
fn burrow_dirty(pod: &Pod, map: &Map) -> bool {
    let burrow = burrow(&pod.1, map);
    burrow.iter().filter(|c| *c != &pod.1 && **c != '.').count() > 0
}

/// Now we only need a function that returns a vec of possible moves, along with the cost.
pub fn moves(map: &Map) -> Vec<(Map, u32)> {
    let deep = map.len() > 15;
    let mut moves = vec![];
    // determining moves is quite easy
    // for each index where a pod can be
    // each possible move has this structure:
    // vec![route_indexes]
    // each route index should be empty, the last route index is the new position and the count
    // of moves is the cost.
    pods_iter(map)
        .filter(|pod| may_move(pod, map))
        .for_each(|pod| {
            // if the pod is in the hallway, and the burrow is 'dirty', skip
            if in_hallway(&pod) && burrow_dirty(&pod, &map) {
                return;
            }
            let routes = routes_from(&pod, deep);

            // get lowest empty burrow indexes
            let lowest_burrows = get_lowest_empty_burrows(&map);
            // remove any routes that have a burrow as a destination (i >= 7) which is not
            // in the lowest burrows
            let routes = routes
                .iter()
                .filter(|r| r.last().unwrap() < &7 || lowest_burrows.contains(r.last().unwrap()))
                .cloned()
                .collect::<Vec<Route>>();

            for route in routes {
                if route_clear(&route, map) {
                    let mut new_map = map.clone();
                    let (from, c) = pod;
                    let to = *route.last().unwrap();
                    new_map.replace_range(from..from + 1, ".");
                    new_map.replace_range(to..to + 1, &c.to_string());

                    let pod_energy = energy(&pod);
                    let energy = route_steps(&route) * pod_energy;
                    moves.push((new_map, energy));
                }
            }
        });
    moves
}

/// This function takes in the puzzle input and returns a map representation
/// This only supports puzzles in the start position. It defaults to an empty hallway.
pub fn parse(input: &str) -> Map {
    let mut map = String::from(".......");

    // disregard first two lines
    let input = input.lines().skip(2).collect::<Vec<&str>>();
    // remove the last line
    let input = input[..input.len() - 1].to_vec();

    // now for each line, go over the chars and add any [A-D] we find to the map.
    for line in input {
        for c in line.chars() {
            if c.is_alphabetic() {
                map.push(c);
            }
        }
    }

    map
}

/// Solve the puzzle by finding the shortest path to the win map.
/// This is a naive approach using a queue. It can be used to solve the first part, but not really
/// for the second part.
pub fn solve(map: &Map, win_map: &Map) -> u32 {
    // use a breadth first search to find the shortest path
    // to the win map
    let mut queue = vec![(map.clone(), 0)];
    let mut visited: HashMap<Map, u32> = HashMap::new();
    let mut min_energy = std::u32::MAX;

    while let Some((map, energy)) = queue.pop() {
        if map == *win_map {
            min_energy = min_energy.min(energy);
            continue;
        }
        let moves = moves(&map);
        for (new_map, new_energy) in moves {
            if !visited.contains_key(&new_map) {
                visited.insert(new_map.clone(), new_energy);
                queue.push((new_map, energy + new_energy));
            } else {
                // if the new energy is less than the old energy, we can update the energy
                if new_energy < *visited.get(&new_map).unwrap() {
                    visited.insert(new_map.clone(), new_energy);
                    queue.push((new_map, energy + new_energy));
                }
            }
        }

        // sort the queue by energy, highest first
        queue.sort_by(|a, b| b.1.cmp(&a.1));
    }

    min_energy
}

/// Solve the shortest path to the win map using the Dijkstra algorithm from the `pathfinding`
/// crate. It is a dramatic improvement over the naive approach.
fn solve_dijkstra(map: &Map, win_map: &Map) -> u32 {
    let result = dijkstra(map, moves, |map| map == win_map);
    if let Some((path, energy)) = result {
        for map in path.iter() {
            print_map(map);
        }
        energy
    } else {
        0
    }
}

pub fn main() {
    let _input_a = r#"#############
#...........#
###B#C#A#D###
  #B#C#D#A#
  #########
"#;
    let input_b = r#"#############
#...........#
###B#C#A#D###
  #D#C#B#A#
  #D#B#A#C#
  #B#C#D#A#
  #########
"#;

    let map = parse(input_b);
    let solution = solve_dijkstra(&map, &String::from(WIN_MAP_XL));

    println!("Day 23a: {solution}");
}

/// Here starts the testing
/// 
/// The tests are a bit verbose, but they are necessary to make sure the functions are working
/// correctly, and most test cases are there to help detect and fix bugs.
///
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse() {
        let input = r#"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
"#;
        let map = parse(input);
        assert_eq!(map, ".......ABCDABCD");
    }

    #[test]
    /// Test the example to see if it finds the moves that are suggested.
    fn test_moves_1() {
        let m = moves(&".......BCBDADCA".to_string());

        // the list of moves should contain the tuple ("..B....BC.DADCA", 40)
        assert!(m.contains(&("..B....BC.DADCA".to_string(), 40)));
    }

    #[test]
    fn test_moves_2() {
        let m = moves(&"..B....BC.DADCA".to_string());
        // the inbetween step for the example is ..BC...B..DADCA 200
        assert!(m.contains(&("..BC...B..DADCA".to_string(), 200)));

        let m = moves(&"..BC...B..DADCA".to_string());
        assert!(m.contains(&("..B....B.CDADCA".to_string(), 200)));
    }

    #[test]
    fn test_moves_3() {
        let m = moves(&"..B....B.CDADCA".to_string());
        assert!(m.contains(&("..BD...B.CDA.CA".to_string(), 3000)));

        let m = moves(&"..BD...B.CDA.CA".to_string());
        assert!(m.contains(&("...D...B.CDABCA".to_string(), 30)));
    }

    #[test]
    fn test_moves_4() {
        let m = moves(&"...D...B.CDABCA".to_string());
        assert!(m.contains(&("..BD.....CDABCA".to_string(), 20)));

        let m = moves(&"..BD.....CDABCA".to_string());
        assert!(m.contains(&("...D....BCDABCA".to_string(), 20)));
    }

    #[test]
    fn test_moves_5() {
        let m = moves(&"...D....BCDABCA".to_string());
        assert!(m.contains(&("...DD...BC.ABCA".to_string(), 2000)));

        let m = moves(&"...DD...BC.ABCA".to_string());
        assert!(m.contains(&("...DDA..BC.ABC.".to_string(), 3)));
    }

    #[test]
    fn test_moves_6() {
        let m = moves(&"...DDA..BC.ABC.".to_string());
        assert!(m.contains(&("...D.A..BC.ABCD".to_string(), 3000)));

        let m = moves(&"...D.A..BC.ABCD".to_string());
        assert!(m.contains(&(".....A..BCDABCD".to_string(), 4000)));
    }

    #[test]
    fn test_moves_7() {
        let m = moves(&".....A..BCDABCD".to_string());
        assert!(m.contains(&(".......ABCDABCD".to_string(), 8)));
    }

    #[rstest]
    #[case(".......BCBDDCBADBACADCA", "......DBCB.DCBADBACADCA")]
    #[case("......DBCB.DCBADBACADCA", "A.....DBCB.DCB.DBACADCA")]
    #[case("A.....DBCB.DCB.DBACADCA", "A....BDBC..DCB.DBACADCA")]
    #[case("A....BDBC..DCB.DBACADCA", "A...BBDBC..DC..DBACADCA")]
    #[case("A....BDBC..DCB.DBACADCA", "A...BBDBC..DC..DBACADCA")]
    #[case("A...BBDBC..DC..DBACADCA", "AA..BBDBC..DC..DB.CADCA")]
    #[case("AA..BBDBC..DC..DB.CADCA", "AA.CBBDB...DC..DB.CADCA")]
    #[case("AA.CBBDB...DC..DB.CADCA", "AA..BBDB...DC..DBCCADCA")]
    #[case("AA..BBDB...DC..DBCCADCA", "AA.CBBDB...D...DBCCADCA")]
    #[case("AA.CBBDB...D...DBCCADCA", "AA..BBDB...D.C.DBCCADCA")]
    #[case("AA..BBDB...D.C.DBCCADCA", "AA.BBBDB...D.C.D.CCADCA")]
    #[case("AA.BBBDB...D.C.D.CCADCA", "AADBBBDB...D.C.D.CCA.CA")]
    #[case("AADBBBDB...D.C.D.CCA.CA", "AAD.BBDB...D.C.D.CCABCA")]
    #[case("AAD.BBDB...D.C.D.CCABCA", "AAD..BDB...D.C.DBCCABCA")]
    #[case("AAD..BDB...D.C.DBCCABCA", "AAD...DB...DBC.DBCCABCA")]
    #[case("AAD...DB...DBC.DBCCABCA", "AAD.C.DB...DBC.DBC.ABCA")]
    #[case("AAD.C.DB...DBC.DBC.ABCA", "AAD...DB.C.DBC.DBC.ABCA")]
    #[case("AAD...DB.C.DBC.DBC.ABCA", "AAD..ADB.C.DBC.DBC.ABC.")]
    #[case("AAD..ADB.C.DBC.DBC.ABC.", "AA...ADB.C.DBC.DBC.ABCD")]
    #[case("AA...ADB.C.DBC.DBC.ABCD", "AAB..AD..C.DBC.DBC.ABCD")]
    #[case("AAB..AD..C.DBC.DBC.ABCD", "AA...AD.BC.DBC.DBC.ABCD")]
    #[case("AA...AD.BC.DBC.DBC.ABCD", "AA..DAD.BC..BC.DBC.ABCD")]
    #[case("AA..DAD.BC..BC.DBC.ABCD", "AA...AD.BC..BC.DBCDABCD")]
    #[case("AA...AD.BC..BC.DBCDABCD", "AAD..AD.BC..BC..BCDABCD")]
    #[case("AAD..AD.BC..BC..BCDABCD", "A.D..AD.BC..BC.ABCDABCD")]
    #[case("A.D..AD.BC..BC.ABCDABCD", "..D..AD.BC.ABC.ABCDABCD")]
    #[case("A.D..AD.BC..BC.ABCDABCD", "..D..AD.BC.ABC.ABCDABCD")]
    #[case("..D..AD.BC.ABC.ABCDABCD", ".....AD.BC.ABCDABCDABCD")]
    #[case(".....AD.BC.ABCDABCDABCD", "......DABC.ABCDABCDABCD")]
    #[case("......DABC.ABCDABCDABCD", ".......ABCDABCDABCDABCD")]
    fn test_deep_moves(#[case] from: Map, #[case] to: Map) {
        let m = moves(&from);
        println!("from:");
        print_map(&from);
        println!("to:");
        print_map(&to);
        println!("moves:"); 
        for (map, _) in &m {
            print_map(map);
        }
        assert!(
            m.iter().filter(|(map, _)| *map == to).count() > 0,
            "map: {from} should have a move to {to}"
        );
    }

    #[rstest]
    #[case(".....A..BCDABCD", 8)]
    #[case("...DDA..BC.ABC.", 7000 + 8)]
    #[case("...D....BCDABCA", 2003 + 7000 + 8)]
    fn test_solve_example(#[case] map: &str, #[case] energy: u32) {
        assert_eq!(
            solve(&map.to_string(), &String::from(WIN_MAP)),
            energy,
            "map: {map} should cost energy {energy}"
        );
    }

    #[test]
    fn test_moves_to_lowest_burrow() {
        // from a hallway into a burrow, the pod should move in the bottom most slot
        let m = moves(&"...DDA..BC.ABC.".to_string());
        assert!(
            m.contains(&("...D.A..BC.ABCD".to_string(), 3000)),
            "A pod should always move into the lowest empty burrow"
        );
    }

    #[test]
    fn test_pod_does_not_move_to_dirty_burrow() {
        // from a hallway into a burrow, the pod should move in the bottom most slot
        let m = moves(&"...DD...BC.ABCA".to_string());
        assert!(
            !m.contains(&("...D....BCDABCA".to_string(), 3000)),
            "A pod should never move into a burrow that contains foreign pods"
        );
    }

    #[test]
    fn test_burrow_function() {
        let map = ".......BCBDADCA".to_string();
        assert_eq!(burrow(&'A', &map), vec!['B', 'A']);
        assert_eq!(burrow(&'B', &map), vec!['C', 'D']);
        assert_eq!(burrow(&'C', &map), vec!['B', 'C']);
        assert_eq!(burrow(&'D', &map), vec!['D', 'A']);
    }

    #[test]
    fn test_may_move() {
        let map = ".......BCBDADCD".to_string();
        assert_eq!(may_move(&(7, 'B'), &map), true);
        assert_eq!(may_move(&(8, 'C'), &map), true);
        assert_eq!(may_move(&(9, 'B'), &map), true);
        assert_eq!(may_move(&(10, 'D'), &map), false);

        let map = "..B....BC.DADCA".to_string();
        assert_eq!(may_move(&(8, 'C'), &map), true);
        assert_eq!(may_move(&(13, 'C'), &map), false);

        let map = "AA.CBBDB...DC..DB.CADCA".to_string();
        assert_eq!(may_move(&(3, 'C'), &map), true);
    }

    #[test]
    fn test_routes_from() {
        let pod = (0, 'A');
        let routes = routes_from(&pod, false);
        assert_eq!(routes.len(), 2);
        assert_eq!(routes, vec![vec![1, 7], vec![1, 7, 11]]);
    }

    #[test]
    fn test_trace() {
        assert_eq!(trace(0, 7), vec![1, 7]);
        assert_eq!(trace(0, 11), vec![1, 7, 11]);
        assert_eq!(trace(1, 7), vec![7]);
        assert_eq!(trace(1, 11), vec![7, 11]);
        assert_eq!(trace(2, 7), vec![7]);
        assert_eq!(trace(2, 11), vec![7, 11]);
        assert_eq!(trace(3, 7), vec![2, 7]);
        assert_eq!(trace(3, 11), vec![2, 7, 11]);
        assert_eq!(trace(4, 7), vec![3, 2, 7]);
        assert_eq!(trace(4, 11), vec![3, 2, 7, 11]);
        assert_eq!(trace(8, 3), vec![3]);
        assert_eq!(trace(5, 7), vec![4, 3, 2, 7]);
    }

    #[rstest]
    // from 0
    #[case(vec![1, 7], 3)]
    #[case(vec![1, 2, 8], 5)]
    #[case(vec![1, 2, 3, 9], 7)]
    #[case(vec![1, 2, 3, 4, 10], 9)]
    // from 1
    #[case(vec![7], 2)]
    #[case(vec![2, 8], 4)]
    #[case(vec![2, 3, 9], 6)]
    #[case(vec![2, 3, 4, 10], 8)]
    // from 2
    #[case(vec![8], 2)]
    #[case(vec![3, 9], 4)]
    #[case(vec![3, 4, 10], 6)]
    // from 3
    #[case(vec![9], 2)]
    #[case(vec![4, 10], 4)]
    #[case(vec![2, 7], 4)]
    // from 4
    #[case(vec![10], 2)]
    #[case(vec![3, 8], 4)]
    #[case(vec![3, 2, 7], 6)]
    // from 5
    #[case(vec![4, 9], 4)]
    #[case(vec![4, 3, 8], 6)]
    #[case(vec![4, 3, 2, 1], 8)]
    // from 6
    #[case(vec![5, 10], 3)]
    #[case(vec![5, 4, 9], 5)]
    #[case(vec![5, 4, 3, 8], 7)]
    #[case(vec![5, 4, 3, 2, 1], 9)]
    // from 7 (burrow)
    #[case(vec![1], 2)]
    #[case(vec![1, 0], 3)]
    #[case(vec![2], 2)]
    #[case(vec![2, 3], 4)]
    #[case(vec![2, 3, 4], 6)]
    #[case(vec![2, 3, 4, 5], 8)]
    #[case(vec![2, 3, 4, 5, 6], 9)]
    // from 8 (burrow)
    #[case(vec![2, 1], 4)]
    #[case(vec![2, 1, 0], 5)]
    #[case(vec![3], 2)]
    #[case(vec![3, 4], 4)]
    #[case(vec![3, 4, 5], 6)]
    #[case(vec![3, 4, 5, 6], 7)]
    // some cases for deeper in the burrow
    // from 11
    #[case(vec![7, 1], 3)]
    #[case(vec![7, 1, 0], 4)]
    #[case(vec![7, 2, 3], 5)]
    // from 16 (B level 3)
    #[case(vec![12, 8, 3], 4)]
    // from 21 (C level 4)
    #[case(vec![17, 13, 9, 4], 5)]
    fn test_route_steps(#[case] route: Route, #[case] steps: u32) {
        // from 0
        assert_eq!(
            route_steps(&route),
            steps,
            "route {:?} should have {} steps",
            route,
            steps
        );
    }
}
