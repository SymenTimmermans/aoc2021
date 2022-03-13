// use vecdeque
use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

/// Lets try this again with a different approach.
/// And the nalgebra library.

// For now, allow dead code
#[allow(dead_code)]
// allow unused variables for now
#[allow(unused_variables)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

// implement add for Vector3
impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// implement debug for Vector3
impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

// implement sub for Vector3
impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[rustfmt::skip]
const ROTATIONS: [(i32, i32, i32); 48] = [
    (1, 2, 3), (1, 3, 2), (2, 1, 3), (2, 3, 1), (3, 1, 2), (3, 2, 1),
    (1, 2, -3), (1, 3, -2), (2, 1, -3), (2, 3, -1), (3, 1, -2), (3, 2, -1),
    (1, -2, 3), (1, -3, 2), (2, -1, 3), (2, -3, 1), (3, -1, 2), (3, -2, 1),
    (1, -2, -3), (1, -3, -2), (2, -1, -3), (2, -3, -1), (3, -1, -2), (3, -2, -1),
    (-1, 2, 3), (-1, 3, 2), (-2, 1, 3), (-2, 3, 1), (-3, 1, 2), (-3, 2, 1),
    (-1, 2, -3), (-1, 3, -2), (-2, 1, -3), (-2, 3, -1), (-3, 1, -2), (-3, 2, -1),
    (-1, -2, 3), (-1, -3, 2), (-2, -1, 3), (-2, -3, 1), (-3, -1, 2), (-3, -2, 1),
    (-1, -2, -3), (-1, -3, -2), (-2, -1, -3), (-2, -3, -1), (-3, -1, -2), (-3, -2, -1),
];

impl Vector3 {
    fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn zero() -> Vector3 {
        Vector3 { x: 0, y: 0, z: 0 }
    }

    fn default_rotation() -> Vector3 {
        Vector3 { x: 1, y: 2, z: 3 }
    }

    fn magnitude_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn abs(&self) -> Vector3 {
        Vector3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn distance(&self, other: &Vector3) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn get_axis(&self, axis: i32) -> i32 {
        match axis {
            1 => self.x,
            2 => self.y,
            3 => self.z,
            -1 => -self.x,
            -2 => -self.y,
            -3 => -self.z,
            _ => panic!("Invalid axis"),
        }
    }

    fn from_tuple(tuple: (i32, i32, i32)) -> Vector3 {
        Vector3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }

    fn rotate_tuple(&self, tuple: (i32, i32, i32)) -> Vector3 {
        Vector3::from_tuple((
            self.get_axis(tuple.0),
            self.get_axis(tuple.1),
            self.get_axis(tuple.2),
        ))
    }

    fn rotate(&self, rot: &Vector3) -> Vector3 {
        Vector3::new(
            self.get_axis(rot.x),
            self.get_axis(rot.y),
            self.get_axis(rot.z),
        )
    }

    fn inverse_rotate(&self, rot: &Vector3) -> Vector3 {
        let mut base = Vector3::zero();
        // the first item of rot determines where the x is lead from
        match rot.x {
            1 => {
                base.x = self.x;
            }
            2 => {
                base.y = self.x;
            }
            3 => {
                base.z = self.x;
            }
            -1 => {
                base.x = -self.x;
            }
            -2 => {
                base.y = -self.x;
            }
            -3 => {
                base.z = -self.x;
            }
            _ => panic!("Invalid axis"),
        }
        // the second item of rot determines where the y is lead from
        match rot.y {
            1 => {
                base.x = self.y;
            }
            2 => {
                base.y = self.y;
            }
            3 => {
                base.z = self.y;
            }
            -1 => {
                base.x = -self.y;
            }
            -2 => {
                base.y = -self.y;
            }
            -3 => {
                base.z = -self.y;
            }
            _ => panic!("Invalid axis"),
        }
        match rot.z {
            1 => {
                base.x = self.z;
            }
            2 => {
                base.y = self.z;
            }
            3 => {
                base.z = self.z;
            }
            -1 => {
                base.x = -self.z;
            }
            -2 => {
                base.y = -self.z;
            }
            -3 => {
                base.z = -self.z;
            }
            _ => panic!("Invalid axis"),
        }
        // the third item of rot determines where the z is lead from
        // return base
        base
    }

    fn inverse_rotate_tuple(&self, tuple: (i32, i32, i32)) -> Vector3 {
        self.inverse_rotate(&Vector3::from_tuple(tuple))
    }
}

#[derive(Debug, Clone, Copy)]
struct Beacon {
    // The position of the beacon.
    pos: Vector3,
    // The distances to the two closest neighbors.
    // Sorted in ascending order.
    // This can be used to match beacons across different scanners.
    close_dist: (Option<i32>, Option<i32>),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Scanner {
    nr: usize,
    pos: Option<Vector3>,
    rot: Option<Vector3>,
    beacons: Vec<Beacon>,
}

impl Scanner {
    pub fn local_to_world_pos(&self, p: Vector3) -> Vector3 {
        // de-rotate the pos
        let p = if let Some(rot) = self.rot {
            p.inverse_rotate(&rot)
        } else {
            p
        };
        // add world pos to p
        if let Some(pos) = self.pos {
            pos + p
        } else {
            p
        }
    }

    pub fn world_pos(&self) -> Vector3 {
        self.pos.unwrap_or_else(Vector3::zero)
    }

    pub fn corrected_beacon_positions(&self) -> Vec<Vector3> {
        self.beacons
            .iter()
            .map(|b| {
                // beacon absolute position is the beacon position plus the scanner position
                b.pos.inverse_rotate(&self.rot.unwrap()) + self.pos.unwrap()
            })
            .collect()
    }

    pub fn distance_calc(&mut self) {
        // clone the beacons vector, so we can use it as reference for calculating the distance
        let beacon_copy = self.beacons.clone();

        // enumerate over the beacons
        self.beacons.iter_mut().enumerate().for_each(|(_, b)| {
            // calculate the distance to the closest neighbor
            let mut closest_dist = None;
            let mut second_closest_dist = None;
            for beacon in &beacon_copy {
                if beacon.pos == b.pos {
                    continue;
                }
                let dist = (b.pos - beacon.pos).magnitude_squared();
                if closest_dist.is_none() || dist < closest_dist.unwrap() {
                    second_closest_dist = closest_dist;
                    closest_dist = Some(dist);
                } else if second_closest_dist.is_none() || dist < second_closest_dist.unwrap() {
                    second_closest_dist = Some(dist);
                }
            }
            b.close_dist = (closest_dist, second_closest_dist);
        });
    }

    pub fn likely_rotation_and_pos(&self, ref_scanner: &Scanner) -> Option<(Vector3, Vector3)> {
        let matching_beacons = distance_based_matching_beacons(self, ref_scanner);
        // what else do we want to know about this scanner, compared to the reference scanner?
        // if the number of matching beacons is 12, We know that these two scanners see a lot of the same beacons.
        if matching_beacons.len() < 12 {
            return None;
        }

        // Now that we have matching beacon pairs, we can try to find the position of the scanner.
        let likely_rotations = ROTATIONS.iter().filter_map(|&r| {
            // iterate over the matching pairs to find the position of the scanner based on this rotation
            let unique_positions = matching_beacons
                .iter()
                .map(|&(b, _, p)| p - b.pos.inverse_rotate_tuple(r))
                .dedup()
                .collect::<Vec<_>>();

            // if there is only one unique position, we can use this as the position of the scanner
            if unique_positions.len() == 1 {
                Some((Vector3::from_tuple(r), unique_positions[0]))
            } else {
                None
            }
        });

        // collect likely rotations
        let likely_rotations = likely_rotations.collect::<Vec<_>>();
        // if theres one in there, return it
        if likely_rotations.len() == 1 {
            return Some(likely_rotations[0]);
        }
        None
    }
}

fn distance_based_matching_beacons(
    scanner: &Scanner,
    ref_scanner: &Scanner,
) -> Vec<(Beacon, Beacon, Vector3)> {
    // loop through the beacons of the scanner
    scanner
        .beacons
        .iter()
        .filter_map(|&b| {
            // find one beacon in the ref_scanner that has the same close_dist
            ref_scanner
                .beacons
                .iter()
                .find(|ref_b| ref_b.close_dist == b.close_dist)
                .map(|ref_b| (b, *ref_b, ref_scanner.local_to_world_pos(ref_b.pos)))
        })
        .collect()
}

fn parse_scanner(input: &str) -> Scanner {
    // split the input into lines
    let lines: Vec<&str> = input.split('\n').collect();

    // get the first line for the number
    let nr_str = lines[0].split(' ').nth(2).unwrap();
    let nr = nr_str.parse::<u32>().unwrap();
    // get the beacons from the remaining lines
    let mut beacons = Vec::new();
    for line in lines.iter().take(lines.len()).skip(1) {
        if line.is_empty() {
            continue;
        }

        // parse coordinates for beacons from the lines
        let coords: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        beacons.push(Beacon {
            pos: Vector3::new(coords[0], coords[1], coords[2]),
            close_dist: (None, None),
        });
    }
    // create the scanner
    Scanner {
        nr: nr as usize,
        pos: None,
        rot: None,
        beacons,
    }
}

fn read_scanners(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(parse_scanner).collect()
}

fn main() {
    let mut scanners = read_scanners(include_str!("../../input/day19.txt"));

    // set the first scanner to 0,0,0 and 1,2,3 as reference
    scanners[0].pos = Some(Vector3::zero());
    scanners[0].rot = Some(Vector3::new(1, 2, 3));

    scanners.iter_mut().for_each(|s| s.distance_calc());

    // put the ref_scanner in the ref_queue
    let mut ref_queue: VecDeque<Scanner> = VecDeque::new();
    ref_queue.push_back(scanners[0].clone());

    // as long as there are scanners left that don't have an initialized pos
    while scanners.iter().any(|s| s.pos.is_none()) {
        // if the ref_queue is empty, panic, because we can't solve this
        if ref_queue.is_empty() {
            panic!("No reference scanners found");
        }

        // get the first scanner from the ref_queue
        let ref_scanner = ref_queue.pop_front().unwrap();

        // loop through the scanners
        scanners
            .iter_mut()
            .filter(|s| s.pos.is_none())
            .for_each(|s| {
                // get likely rotation and pos for the scanner
                if let Some((rot, pos)) = s.likely_rotation_and_pos(&ref_scanner) {
                    // set the pos and rot of the scanner
                    s.pos = Some(pos);
                    s.rot = Some(rot);
                    // this scanner can now be used as a reference scanner
                    ref_queue.push_back(s.clone());
                    println!(
                        "S{} >>> REF S{} -> {:?}, {:?}",
                        s.nr, ref_scanner.nr, s.pos, s.rot
                    );
                }
            });
    }
    // All scanners have positions and rotations now.
    // We should now build up a list of beacons with absolute positions (relative to reference scanner).
    // And deduplicate this list. This will tell us how many beacons there truly are.
    let mut beacons: HashSet<Vector3> = HashSet::new();
    for scanner in scanners.iter() {
        beacons.extend(scanner.corrected_beacon_positions());
    }
    // print the number of beacons:
    println!("Nr of beacons: {}", beacons.len());

    // part 2: largest manhattan distance
    let distances = scanners
        .iter()
        .combinations(2)
        .map(|c| c[0].world_pos().distance(&c[1].world_pos()));

    println!("Largest manhattan distance: {}", distances.max().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input() {
        let input = r#"--- scanner 0 ---
-1,2,3

--- scanner 1 ---
4,-5,6
7,8,-9
"#;
        let scanners = read_scanners(input);

        // check the number of scanners
        assert_eq!(scanners.len(), 2);

        // check the first scanner
        assert_eq!(scanners[0].nr, 0);
        // the scanner should have 1 beacon
        assert_eq!(scanners[0].beacons.len(), 1);
        // the beacon should be at position (-1, 2, 3)
        assert_eq!(scanners[0].beacons[0].pos, Vector3::new(-1, 2, 3));

        // check the second scanner
        assert_eq!(scanners[1].nr, 1);
        // the scanner should have 2 beacons
        assert_eq!(scanners[1].beacons.len(), 2);

        // the first beacon should be at position (4, -5, 6)
        assert_eq!(scanners[1].beacons[0].pos, Vector3::new(4, -5, 6));
        // the second beacon should be at position (7, 8, -9)
        assert_eq!(scanners[1].beacons[1].pos, Vector3::new(7, 8, -9));
    }

    #[test]
    // Within a scanner, the beacons are of certain distance to each other.
    // From any beacon, the distance to the closest beacon is the same.
    // The distance to the second closest beacon is always larger than the distance to the closest beacon.
    // These two distances are called the "close distances".
    fn distance_calc() {
        let mut scanner = Scanner {
            nr: 0,
            pos: Some(Vector3::zero()),
            rot: Some(Vector3::new(0, 0, 0)),
            beacons: Vec::new(),
        };
        scanner.beacons.push(Beacon {
            pos: Vector3::zero(),
            close_dist: (None, None),
        });
        scanner.beacons.push(Beacon {
            pos: Vector3::new(2, 0, 0),
            close_dist: (None, None),
        });
        scanner.beacons.push(Beacon {
            pos: Vector3::new(0, 5, 0),
            close_dist: (None, None),
        });
        scanner.beacons.push(Beacon {
            pos: Vector3::new(0, 0, 15),
            close_dist: (None, None),
        });
        scanner.distance_calc();
        assert_eq!(scanner.beacons[0].close_dist, (Some(4), Some(25)));
        assert_eq!(scanner.beacons[1].close_dist, (Some(4), Some(29)));
        assert_eq!(scanner.beacons[2].close_dist, (Some(25), Some(29)));
        assert_eq!(scanner.beacons[3].close_dist, (Some(225), Some(229)));
    }

    #[test]
    fn determine_neighbours() {
        let input = r#"--- scanner 0 ---
0,0,0
0,0,2
0,0,5
"#;
        let mut scanners = read_scanners(input);

        // check the number of scanners
        assert_eq!(scanners.len(), 1);

        // check the first scanner
        assert_eq!(scanners[0].nr, 0);
        // the scanner should have 3 beacons
        assert_eq!(scanners[0].beacons.len(), 3);
        // the first beacon should be at position (0, 0, 0)
        assert_eq!(scanners[0].beacons[0].pos, Vector3::new(0, 0, 0));
        // the second beacon should be at position (0, 0, 2)
        assert_eq!(scanners[0].beacons[1].pos, Vector3::new(0, 0, 2));
        // the third beacon should be at position (0, 0, 5)
        assert_eq!(scanners[0].beacons[2].pos, Vector3::new(0, 0, 5));

        // run the distance calculation
        scanners[0].distance_calc();

        // the first beacon should have the distance to the second beacon
        // and the third beacon
        assert_eq!(scanners[0].beacons[0].close_dist, (Some(4), Some(25)));
        // the second beacon should have the distance to the first beacon
        // and the third beacon
        assert_eq!(scanners[0].beacons[1].close_dist, (Some(4), Some(9)));
        // the third beacon should have the distance to the first beacon
        // and the second beacon
        assert_eq!(scanners[0].beacons[2].close_dist, (Some(9), Some(25)));
    }

    #[test]
    fn invert_rotate() {
        let v1 = Vector3::new(10, 20, 30);

        assert_eq!(
            v1.rotate(&Vector3::default_rotation()),
            Vector3::new(10, 20, 30)
        );

        let rotated = v1.rotate(&Vector3::new(2, 3, 1));
        assert_eq!(rotated, Vector3::new(20, 30, 10));
        let inv_rot = rotated.inverse_rotate(&Vector3::new(2, 3, 1));
        assert_eq!(inv_rot, v1);
    }

    #[test]
    // Test the various claims made in the example.
    fn test_example() {
        // read in the example input
        let mut scanners = read_scanners(include_str!("../../input/day19_ex.txt"));

        // run the distance calculation
        for scanner in scanners.iter_mut() {
            scanner.distance_calc();
        }

        // init first scanner on position (0, 0, 0) and rotation (1, 2, 3)
        scanners[0].pos = Some(Vector3::new(0, 0, 0));
        scanners[0].rot = Some(Vector3::new(1, 2, 3));

        // ----------------------------------------------
        // check matching beacons of overlapping scanners
        // ----------------------------------------------
        let matching_beacons = distance_based_matching_beacons(&scanners[1], &scanners[0]);

        let rpos = matching_beacons.iter().map(|b| b.2).collect::<Vec<_>>();

        // Scanners 0 and 1 have overlapping detection cubes; the 12 beacons they both detect (relative to scanner 0) are at the following coordinates:
        assert_eq!(rpos.len(), 12);

        // -618,-824,-621
        assert!(rpos.contains(&Vector3::new(-618, -824, -621)));
        // -537,-823,-458
        assert!(rpos.contains(&Vector3::new(-537, -823, -458)));
        // -447,-329,318
        assert!(rpos.contains(&Vector3::new(-447, -329, 318)));
        // 404,-588,-901
        assert!(rpos.contains(&Vector3::new(404, -588, -901)));
        // 544,-627,-890
        assert!(rpos.contains(&Vector3::new(544, -627, -890)));
        // 528,-643,409
        assert!(rpos.contains(&Vector3::new(528, -643, 409)));
        // -661,-816,-575
        assert!(rpos.contains(&Vector3::new(-661, -816, -575)));
        // 390,-675,-793
        assert!(rpos.contains(&Vector3::new(390, -675, -793)));
        // 423,-701,434
        assert!(rpos.contains(&Vector3::new(423, -701, 434)));
        // -345,-311,381
        assert!(rpos.contains(&Vector3::new(-345, -311, 381)));
        // 459,-707,401
        assert!(rpos.contains(&Vector3::new(459, -707, 401)));
        // -485,-357,347
        assert!(rpos.contains(&Vector3::new(-485, -357, 347)));

        let spos = matching_beacons.iter().map(|b| b.0.pos).collect::<Vec<_>>();

        // These same 12 beacons (in the same order) but from the perspective of scanner 1 are:
        assert_eq!(spos.len(), 12);

        // 686,422,578
        assert!(spos.contains(&Vector3::new(686, 422, 578)));
        // 605,423,415
        assert!(spos.contains(&Vector3::new(605, 423, 415)));
        // 515,917,-361
        assert!(spos.contains(&Vector3::new(515, 917, -361)));
        // -336,658,858
        assert!(spos.contains(&Vector3::new(-336, 658, 858)));
        // -476,619,847
        assert!(spos.contains(&Vector3::new(-476, 619, 847)));
        // -460,603,-452
        assert!(spos.contains(&Vector3::new(-460, 603, -452)));
        // 729,430,532
        assert!(spos.contains(&Vector3::new(729, 430, 532)));
        // -322,571,750
        assert!(spos.contains(&Vector3::new(-322, 571, 750)));
        // -355,545,-477
        assert!(spos.contains(&Vector3::new(-355, 545, -477)));
        // 413,935,-424
        assert!(spos.contains(&Vector3::new(413, 935, -424)));
        // -391,539,-444
        assert!(spos.contains(&Vector3::new(-391, 539, -444)));
        // 553,889,-390
        assert!(spos.contains(&Vector3::new(553, 889, -390)));

        // -----------------------------------------------------
        // get likely rotation of scanner 1, seen from scanner 0
        // -----------------------------------------------------
        let rotpos = scanners[1].likely_rotation_and_pos(&scanners[0]);

        // assert rot is some
        assert!(rotpos.is_some());

        if let Some((rot, pos)) = rotpos {
            // Because of this, scanner 1 must be at 68,-1246,-43 (relative to scanner 0).
            assert_eq!(pos, Vector3::new(68, -1246, -43));
            // set rotpos to scanner 1
            scanners[1].pos = Some(pos);
            scanners[1].rot = Some(rot);
        }

        // Scanner 4 overlaps with scanner 1;
        let matching_beacons = distance_based_matching_beacons(&scanners[4], &scanners[1]);
        let rpos = matching_beacons.iter().map(|b| b.2).collect::<Vec<_>>();

        // Scanner 4 overlaps with scanner 1; the 12 beacons they both detect (relative to scanner 0) are:

        // 459,-707,401
        assert!(rpos.contains(&Vector3::new(459, -707, 401)));
        // -739,-1745,668
        assert!(rpos.contains(&Vector3::new(-739, -1745, 668)));
        // -485,-357,347
        assert!(rpos.contains(&Vector3::new(-485, -357, 347)));
        // 432,-2009,850
        assert!(rpos.contains(&Vector3::new(432, -2009, 850)));
        // 528,-643,409
        assert!(rpos.contains(&Vector3::new(528, -643, 409)));
        // 423,-701,434
        assert!(rpos.contains(&Vector3::new(423, -701, 434)));
        // -345,-311,381
        assert!(rpos.contains(&Vector3::new(-345, -311, 381)));
        // 408,-1815,803
        assert!(rpos.contains(&Vector3::new(408, -1815, 803)));
        // 534,-1912,768
        assert!(rpos.contains(&Vector3::new(534, -1912, 768)));
        // -687,-1600,576
        assert!(rpos.contains(&Vector3::new(-687, -1600, 576)));
        // -447,-329,318
        assert!(rpos.contains(&Vector3::new(-447, -329, 318)));
        // -635,-1737,486
        assert!(rpos.contains(&Vector3::new(-635, -1737, 486)));

        let rotpos4 = scanners[4].likely_rotation_and_pos(&scanners[1]);
        // assert rotpos4 is some}
        assert!(rotpos4.is_some());

        if let Some((rot, pos)) = rotpos4 {
            // So, scanner 4 is at -20,-1133,1061 (relative to scanner 0).
            assert_eq!(pos, Vector3::new(-20, -1133, 1061));
            scanners[4].pos = Some(pos);
            scanners[4].rot = Some(rot);
        }

        // -----------------------------------------------------
        // Remaining scanners
        // -----------------------------------------------------
        // Following this process, scanner 2 must be at 1105,-1205,1229 (relative to scanner 0)
        let rotpos2 = scanners[2].likely_rotation_and_pos(&scanners[4]);
        assert!(rotpos2.is_some());

        if let Some((rot, pos)) = rotpos2 {
            assert_eq!(pos, Vector3::new(1105, -1205, 1229));
            scanners[2].pos = Some(pos);
            scanners[2].rot = Some(rot);
        }

        // and scanner 3 must be at -92,-2380,-20 (relative to scanner 0).
        let rotpos3 = scanners[3].likely_rotation_and_pos(&scanners[1]);
        assert!(rotpos3.is_some());

        if let Some((rot, pos)) = rotpos3 {
            assert_eq!(pos, Vector3::new(-92, -2380, -20));
            scanners[3].pos = Some(pos);
            scanners[3].rot = Some(rot);
        }
    }
}
