use itertools::Itertools;

/// We need an efficient way to represent the map map in order to use efficient ways of checking
/// possible moves. We have 11 hallway positions, but 4 of them are unusable, so that leaves 7
/// possible positions.
/// We also need to represent the burrows, so we can have a map that sort of looks like this:
///

/// .......ABCDABCD
/// This is also the desired win map.
const WIN_MAP: &str = ".......ABCDABCD";

/// .. . . . .. ABCDABCD
/// Compressed:
/// .......ABCDABCD
/// We can use a string for this
type Map = String;

/// a pod is simply a char in the map
type Pod = (usize, char);

/// a route is a series of indexes not including the start index
type Route = Vec<usize>;

fn pods_iter(map: &Map) -> impl Iterator<Item = Pod> + '_ {
    map.chars()
        .enumerate()
        .filter(|(_, c)| ['A', 'B', 'C', 'D'].contains(c))
}

fn in_burrow(pod: &Pod) -> bool {
    pod.0 >= 7
}

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

fn in_hallway(pod: &Pod) -> bool {
    pod.0 < 7
}

fn energy(pod: &Pod) -> u32 {
    match pod.1 {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

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

    // println!("{pod:?} map:{map} burrow:{burrow:?}");

    // burrow should contain other characters than c for the pod to be allowed to move
    burrow.iter().filter(|x| *x != c && *x != &'.').count() > 0
}

fn trace(from: usize, to: usize) -> Route {
    let mut r = vec![];
    if to >= 7 {
        // going to a burrow, but which one
        let b = (to - 7) % 4 + 1;
        // lets go to the top of the burrow
        // 01 2 3 4 56
        // first see if we need to go left or right
        let dir: i8 = if from < b { 1 } else { -1 };
        let mut pos = from;
        while pos != b {
            pos = (pos as i8 + dir) as usize;
            r.push(pos as usize);
        }
        // now let's go down the burrow
        pos += 6;
        r.push(pos);
        while pos != to {
            pos = pos + 4;
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

fn route_steps(route: &Route) -> u32 {
    // count how many of indexes 2, 3, 4, are in the route
    let mut skipped = route
        .iter()
        .filter(|i| **i >= 2 as usize && **i <= 4 as usize)
        .count() as u32;
    // if we haven't skipped, we may have moved into a burrow, so add one for that
    if skipped == 0 {
        skipped = 1;
    }
    route.len() as u32 + skipped
}

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
                routes.push(trace(pod.0, 11));
            }
            'C' => {
                routes.push(trace(pod.0, 9));
                routes.push(trace(pod.0, 11));
            }
            'D' => {
                routes.push(trace(pod.0, 10));
                routes.push(trace(pod.0, 11));
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
    routes.iter().filter(|r| r.len() > 0).cloned().collect()
}

pub fn route_clear(route: &Route, map: &Map) -> bool {
    route.iter().all(|i| map.chars().nth(*i).unwrap() == '.')
}

pub fn print_map(map: &Map) {
    // print the top row
    println!("#############");
    print!("#");
    for i in 0..7 {
        let c = map.chars().nth(i).unwrap();
        print!("{}", c);
        //if i in (1, 2, 3, 4), print an extra .
        if i >= 1 && i <= 4 {
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
            let c = map.chars().nth(i + j).expect("oob");
            print!("{}#", c);
        }
        println!("  ");
        i += 4;
    }
}

/// Now we only need a function that returns a vec of possible moves, along with the cost.
pub fn moves(map: &Map) -> Vec<(Map, u32)> {
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
            let routes = routes_from(&pod, false);
            // println!("pod: {pod:?} routes: {:?}", routes);

            for route in routes {
                if route_clear(&route, map) {
                    // println!("clear route: {:?}", route);
                    let mut new_map = map.clone();
                    let (from, c) = pod;
                    let to = *route.last().unwrap();
                    new_map.replace_range(from..from + 1, ".");
                    new_map.replace_range(to..to + 1, &c.to_string());

                    let pod_energy = energy(&pod);
                    let energy = route_steps(&route) * pod_energy;
                    // println!("{map} > {new_map} route: {route:?}       {} * {} = {}",
                    // route_steps(&route), pod_energy, energy);
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

pub fn solve(input: &str) -> u32 {
    let map = parse(input);

    // use a breadth first search to find the shortest path
    // to the win map
    let mut queue = vec![(map.clone(), 0)];
    let mut visited = vec![map.clone()];
    let mut min_energy = std::u32::MAX;

    while let Some((map, energy)) = queue.pop() {
        if map == WIN_MAP {
            min_energy = min_energy.min(energy);
            continue;
        }
        let moves = moves(&map);
        // print queue size, visited size, new moves, min energy
        println!(
            "queue: {} visited: {} moves: {} min_energy: {}",
            queue.len(),
            visited.len(),
            moves.len(),
            min_energy
        );
        for (new_map, new_energy) in moves {
            if !visited.contains(&new_map) {
                println!("new_map: {new_map} energy: {new_energy}");
                visited.push(new_map.clone());
                queue.push((new_map, energy + new_energy));
            }
        }

        // sort the queue by energy, lowest first
        queue.sort_by_key(|x| x.1);
    }

    min_energy
}

pub fn main() {
    let input = r#"#############
#...........#
###B#C#A#D###
  #B#C#D#A#
  #########
"#;
    let input = r#"#############
#.A.........#
###.#B#C#D###
  #A#B#C#D#
  #########
"#;
    let map = Map::from(".A......BCDABCD");
    // let solution = solve(input);
    print_map(&map);
    // println!("Day 23a: {solution}");
}

/// Here start the testing
#[cfg(test)]
mod tests {
    use super::*;

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

        let input = r#"#############
#.A.........#
###.#B#C#D###
  #A#B#C#D#
  #########
"#;

        let map = parse(input);
        assert_eq!(map, ".A.....BCDABCD");
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
        assert_eq!(trace(2, 7), vec![1, 7]);
        assert_eq!(trace(2, 11), vec![1, 7, 11]);
        assert_eq!(trace(3, 7), vec![2, 1, 7]);
        assert_eq!(trace(3, 11), vec![2, 1, 7, 11]);
        assert_eq!(trace(4, 7), vec![3, 2, 1, 7]);
        assert_eq!(trace(4, 11), vec![3, 2, 1, 7, 11]);
        assert_eq!(trace(8, 3), vec![3]);
    }
}
