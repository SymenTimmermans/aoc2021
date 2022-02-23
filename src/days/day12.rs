use std::{str::FromStr, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Node {
    Start,
    End,
    SmallCave(char),
    BigCave(char)
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

        if s.len() == 1 {
            let c: char = s.chars().next().unwrap();
            if c.is_lowercase() {
                return Ok(Node::SmallCave(c));
            }
            if c.is_uppercase() {
                return Ok(Node::BigCave(c));
            }
        }

        Err(())
    }
}

type Connection = (Node, Node);

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
        let from_list = self.conn.entry(*from).or_insert_with(Vec::new);
        // add to to the list
        from_list.push(*to);

        // get the list of connections for the to node, or insert to, otherwise
        let to_list = self.conn.entry(*to).or_insert_with(Vec::new);
        // add from to the list
        to_list.push(*from);
    }
}

pub fn day12() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_map() {
        let map = Map::from_lines(vec!(
            "start-A".to_string(),
            "A-B".to_string(),
        ).as_slice());

        assert_eq!(map.conn.len(), 3);
    }

}