use std::{str::FromStr, collections::{HashMap, HashSet}, fmt};
use aoc2021::read_strs;

#[derive(PartialEq, Eq, Clone, Hash)]
enum Node {
    Start,
    End,
    SmallCave(String),
    BigCave(String)
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("start") {
            return Ok(Node::Start);
        }

        if s.eq("end") {
            return Ok(Node::End);
        }

        let c: char = s.chars().next().unwrap();
        if c.is_lowercase() {
            return Ok(Node::SmallCave(s.to_string()));
        }
        if c.is_uppercase() {
            return Ok(Node::BigCave(s.to_string()));
        }

        Err(())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Start => write!(f, "start"),
            Node::End => write!(f, "end"),
            Node::SmallCave(c) => write!(f, "{}", c),
            Node::BigCave(c) => write!(f, "{}", c)
        }
    }
}

struct Map {
    conn: HashMap<Node, Vec<Node>>
}



impl Map {
    fn from_lines(lines: &[String]) -> Map {
        // create empty map
        let mut map = Map {
            conn: HashMap::new()
        };
        
        // for each line, run parse_line
        for line in lines {
            map.parse_line(line.clone());
        }

        map
    }

    fn parse_line(&mut self, line: String) {
        let (from, to) = line.split_once("-").unwrap();

        // parse from into node
        let from = from.parse::<Node>().unwrap();
        // parse to into node
        let to = to.parse::<Node>().unwrap();

        self.add_connection(&from, &to);
    }

    fn add_connection(&mut self, from: &Node, to: &Node) {
        // get the list of connections for the from node, or insert from, otherwise
        let from_list = self.conn.entry(from.clone()).or_insert_with(Vec::new);
        // add to to the list
        from_list.push(to.clone());

        // get the list of connections for the to node, or insert to, otherwise
        let to_list = self.conn.entry(to.clone()).or_insert_with(Vec::new);
        // add from to the list
        to_list.push(from.clone());
    }

    fn get_paths(&self) -> Vec<Vec<Node>> {
        // start from the start node, and get all paths
        // that lead to the end node
        let start_connections = self.conn.get(&Node::Start).unwrap();
        let mut paths = Vec::new();
        for node in start_connections {
            println!("Calculating Start -> {:?}", node);
            let mut path = vec![Node::Start, node.clone()];
            self.get_paths_recursive(node, &mut path, &mut paths);
        }

        paths
    }

    fn get_paths_recursive(&self, node: &Node, path: &mut Vec<Node>, paths: &mut Vec<Vec<Node>>) {
        // if we've reached the end node, add the path to the list of paths
        if node == &Node::End {
            paths.push(path.clone());
            return;
        }

        // if we hit a start node, return
        if node == &Node::Start {
            return;
        }

        // get the list of connections for the node
        let connections = self.conn.get(node).unwrap();
        // for each connection, add it to the path, and recurse
        for connection in connections {
            // if the connection is a small cave
            if let Node::SmallCave(_) = connection {
                // if the path already contains this connection, skip it.
                if path.contains(connection) {
                    continue;
                }
            }

            path.push(connection.clone());
            self.get_paths_recursive(connection, path, paths);
            path.pop();
        }
    }

    fn get_paths_p2(&self) -> Vec<Vec<Node>> {
        // start from the start node, and get all paths
        // that lead to the end node
        let start_connections = self.conn.get(&Node::Start).unwrap();
        let mut paths = Vec::new();
        for node in start_connections {
            println!("Calculating Start -> {:?}", node);
            let mut path = vec![Node::Start, node.clone()];
            self.get_paths_p2_recursive(node, &mut path, &mut paths);
        }

        paths
    }

    fn get_paths_p2_recursive(&self, node: &Node, path: &mut Vec<Node>, paths: &mut Vec<Vec<Node>>) {
        // if we've reached the end node, add the path to the list of paths
        if node == &Node::End {
            paths.push(path.clone());
            return;
        }

        // if we hit a start node, return
        if node == &Node::Start {
            return;
        }

        // get the list of connections for the node
        let connections = self.conn.get(node).unwrap();
        // for each connection, add it to the path, and recurse
        for connection in connections {
            // if the connection is a small cave
            if let Node::SmallCave(_) = connection {
                
                // the trick now is that we're able to visit one small cave 
                // twice, so we need to know if we already have a path that
                // includes a "double visit" of a small cave.
                if contains_double_visit(path) {
                    // if so, we can't revisit this small cave, so        
                    // if the path already contains this connection, skip it.
                    if path.contains(connection) {
                        continue;
                    }
                } else {
                    // our path does not yet contain a double visit, so we can
                    // visit this small cave again.
                }

            }

            path.push(connection.clone());
            self.get_paths_p2_recursive(connection, path, paths);
            path.pop();
        }
    }
}

fn contains_double_visit(path: &[Node]) -> bool {
    let mut small_caves = Vec::new();
    for node in path {
        if let Node::SmallCave(c) = node {
            small_caves.push(c);
        }
    }

    // the number of unique small caves in the path should equal the total number of small caves
    small_caves.len() != small_caves.iter().collect::<HashSet<_>>().len()
}


pub fn main() {
    let input = read_strs("input/day12.txt");
    let map = Map::from_lines(&input);
    println!("MAP: {:?}", map.conn);
    let paths = map.get_paths();
    println!("{} PATHS:", paths.len());
    for path in paths {
        println!("{:?}", path);
    }
    println!("Part 2: {} PATHS:", map.get_paths_p2().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_map() {
        let map = Map::from_lines(vec!(
            "start-A".to_string(),
            "A-end".to_string(),
        ).as_slice());

        assert_eq!(map.conn.len(), 3);

        // test paths
        let paths = map.get_paths();
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_paths() {
        let map = Map::from_lines(vec!(
            "start-A".to_string(),
            "A-b".to_string(),
            "A-c".to_string(),
            "b-C".to_string(),
            "c-end".to_string(),
        ).as_slice());

        let paths = map.get_paths();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_example1() {
        let lines = read_strs("input/day12_ex.txt");
        let map = Map::from_lines(&lines);
        let paths = map.get_paths();
        assert_eq!(paths.len(), 10);
    }

    #[test]
    fn test_example2() {
        let lines = read_strs("input/day12_ex2.txt");
        let map = Map::from_lines(&lines);
        let paths = map.get_paths();
        assert_eq!(paths.len(), 19);
    }

    #[test]
    fn test_example3() {
        let lines = read_strs("input/day12_ex3.txt");
        let map = Map::from_lines(&lines);
        let paths = map.get_paths();
        assert_eq!(paths.len(), 226);
    }

    #[test]
    fn test_example1_p2() {
        let lines = read_strs("input/day12_ex.txt");
        let map = Map::from_lines(&lines);
        let paths = map.get_paths_p2();
        assert_eq!(paths.len(), 36);
    }

    #[test]
    fn test_example2_p2() {
        let lines = read_strs("input/day12_ex2.txt");
        let map = Map::from_lines(&lines);
        let paths = map.get_paths_p2();
        assert_eq!(paths.len(), 103);
    }

    #[test]
    fn test_example3_p2() {
        let lines = read_strs("input/day12_ex3.txt");
        let map = Map::from_lines(&lines);
        let paths = map.get_paths_p2();
        assert_eq!(paths.len(), 3509);
    }
}