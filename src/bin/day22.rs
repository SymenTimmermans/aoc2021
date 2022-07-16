use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

type Position = (i32, i32, i32);
type Reactor = HashSet<Position>;

fn apply_step(reactor: &mut Reactor, step: &Step) {
    // if this step is not in working_range, skip it
    if !step.in_working_range() {
        return;
    }
    for x in step.cuboid.0 .0..step.cuboid.0 .1 {
        for y in step.cuboid.1 .0..step.cuboid.1 .1 {
            for z in step.cuboid.2 .0..step.cuboid.2 .1 {
                let position = (x, y, z);
                match step.value {
                    true => {
                        reactor.insert(position);
                    }
                    false => {
                        reactor.remove(&position);
                    }
                }
            }
        }
    }
}

/// to 51 to reflect non-inclusive range
const WORKING_RANGE: (i32, i32) = (-50, 51);

/// Range is non-inclusive
type Range = (i32, i32);
type Cuboid = (Range, Range, Range);

struct Step {
    value: bool,
    cuboid: Cuboid,
}

impl FromStr for Step {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, cuboid) = s.split_once(' ').unwrap();
        let ranges = cuboid.split(',');
        // each range is a pair of digits separated by '..'
        let mut cuboid = ranges.map(|r| {
            let (start, end) = r.split_once("..").unwrap();
            let (_, start) = start.split_once("=").unwrap();
            (
                start.parse::<i32>().unwrap(),
                end.parse::<i32>().unwrap() + 1, // because non-inclusive
            )
        });
        let cuboid = (
            cuboid.next().unwrap(),
            cuboid.next().unwrap(),
            cuboid.next().unwrap(),
        );
        Ok(Step {
            value: match value {
                "on" => true,
                "off" => false,
                _ => panic!("invalid value"),
            },
            cuboid,
        })
    }
}

impl Step {
    /// returns true if the range of each axis of this step overlaps the WORKING_RANGE
    fn in_working_range(&self) -> bool {
        fn ranges_overlap(r1: (i32, i32), r2: (i32, i32)) -> bool {
            r1.0 <= r2.1 && r2.0 <= r1.1
        }
        let (x, y, z) = self.cuboid;
        ranges_overlap(x, WORKING_RANGE)
            && ranges_overlap(y, WORKING_RANGE)
            && ranges_overlap(z, WORKING_RANGE)
    }
}

fn read_steps(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|line| Step::from_str(line).unwrap())
        .collect()
}

type Reactor2 = Vec<Cuboid>;

/// Applies a step to a reactor.
fn apply_step2(reactor: &mut Reactor2, step: &Step) {
    // before we can add this cuboid, we need to subtract the overlap from every known cuboid in the reactor.
    // the subtraction process is done by the `subtract_cuboid` function, which will return a list of cuboids that
    // should replace the source cuboid. These cuboids make up a shape that that remains after subtracting this new
    // cuboid. These cuboids should be added to the reactor.
    let mut new_cuboids = Vec::new();
    for cuboid in reactor.iter() {
        let mut new_cuboids_for_cuboid = subtract_cuboid(*cuboid, step.cuboid);
        new_cuboids.append(&mut new_cuboids_for_cuboid);
    }

    // set the new cuboids as the reactor.
    reactor.clear();
    reactor.extend(new_cuboids);

    if step.value {
        reactor.push(step.cuboid);
    }
}

/// This is what I wanted to do all along. A cuboid should be a simple enough shape to be able to slice apart.
fn subtract_cuboid(source: Cuboid, subtract: Cuboid) -> Vec<Cuboid> {
    // If these cuboids don't overlap, there's nothing to do, so just return a Vec with the source cuboid.
    if let Some(overlap) = overlap_cuboid(source, subtract) {
        // if the overlap is the same size as the source cuboid, the new cuboid will go over it entirely.
        // in this case we can return an empty vector.
        if overlap == source {
            return Vec::new();
        }

        // The overlap can't be bigger than the source cuboid, so it should be smaller.
        // Collect the unique values of axis that are in both the source and the overlap.
        let mut points_x = HashSet::new();
        points_x.insert(overlap.0 .0);
        points_x.insert(overlap.0 .1);
        points_x.insert(source.0 .0);
        points_x.insert(source.0 .1);
        let mut points_x = points_x.into_iter().collect::<Vec<_>>();
        // sort points_x
        points_x.sort_unstable();

        let mut points_y = HashSet::new();
        points_y.insert(overlap.1 .0);
        points_y.insert(overlap.1 .1);
        points_y.insert(source.1 .0);
        points_y.insert(source.1 .1);
        let mut points_y = points_y.into_iter().collect::<Vec<_>>();
        // sort points_y
        points_y.sort_unstable();

        let mut points_z = HashSet::new();
        points_z.insert(overlap.2 .0);
        points_z.insert(overlap.2 .1);
        points_z.insert(source.2 .0);
        points_z.insert(source.2 .1);
        let mut points_z = points_z.into_iter().collect::<Vec<_>>();
        // sort points_z
        points_z.sort_unstable();

        // create the new cuboids
        let mut new_cuboids = Vec::new();
        for (x1, x2) in points_x.iter().tuple_windows() {
            for (y1, y2) in points_y.iter().tuple_windows() {
                for (z1, z2) in points_z.iter().tuple_windows() {
                    let new_cuboid = ((*x1, *x2), (*y1, *y2), (*z1, *z2));
                    new_cuboids.push(new_cuboid);
                }
            }
        }

        // if we've done our job, new_cuboids should contain a cuboid with the same dimensions as the subtract cuboid.
        // that one should be removed
        new_cuboids.retain(|cuboid| *cuboid != overlap);
        new_cuboids
    } else {
        return vec![source];
    }
}

/// Returns the overlapping cuboid between two cuboids.
/// If there is no overlap, returns None.
/// If there is overlap, returns the overlapping cuboid.
/// The overlapping cuboid is the intersection of the two cuboids.
/// ```
/// let cuboid1 = ((0,2), (0,2), (0,2));
/// let cuboid2 = ((1,3), (1,3), (1,3));
/// assert_eq!(overlap_cuboid(cuboid1, cuboid2), Some(((1,2), (1,2), (1,2)));
/// ```
///
fn overlap_cuboid(c1: Cuboid, c2: Cuboid) -> Option<Cuboid> {
    let (x1, y1, z1) = c1;
    let (x2, y2, z2) = c2;
    let x_overlap = overlap(x1, x2);
    let y_overlap = overlap(y1, y2);
    let z_overlap = overlap(z1, z2);
    if x_overlap.is_none() || y_overlap.is_none() || z_overlap.is_none() {
        return None;
    }
    let x_overlap = x_overlap.unwrap();
    let y_overlap = y_overlap.unwrap();
    let z_overlap = z_overlap.unwrap();
    Some((x_overlap, y_overlap, z_overlap))
}

/// Returns the overlap of two ranges.
/// Returns None if the ranges do not overlap.
/// Returns Some(overlap) if the ranges overlap.
/// ```
/// assert_eq!(overlap((0, 1), (1, 2)), Some((1, 1)));
/// assert_eq!(overlap((0, 1), (2, 3)), None);
/// ```
fn overlap(r1: Range, r2: Range) -> Option<Range> {
    let start = r1.0.max(r2.0);
    let end = r1.1.min(r2.1);
    if start < end {
        Some((start, end))
    } else {
        None
    }
}

/// Calculate the cuboid size
/// ```
/// let cuboid = ((0,1), (0,1), (0,1));
/// assert_eq!(cuboid_size(cuboid), 8);
///
/// let cuboid = ((1,0), (1,0), (1,0));
/// assert_eq!(cuboid_size(cuboid), -8);
/// ```
pub fn cuboid_size(cuboid: &Cuboid) -> i64 {
    (cuboid.0 .1 - cuboid.0 .0) as i64
        * (cuboid.1 .1 - cuboid.1 .0) as i64
        * (cuboid.2 .1 - cuboid.2 .0) as i64
}

/// Return the number of lit cubes in the reactor
/// Assuming the reactor contains all non-overlapping, lit cuboids,
/// this is the sum of the cuboid sizes.
fn count_cubes(reactor: &[Cuboid]) -> i64 {
    reactor.iter().map(cuboid_size).sum::<i64>()
}

/// Printout the contents of the reactor.
#[allow(dead_code)]
fn print_reactor2(reactor: &[Cuboid]) {
    println!(
        "-------------------- REACTOR ----- P: {} ------",
        reactor.len(),
    );
    reactor
        .iter()
        .enumerate()
        .for_each(|(i, c)| println!("POS {}. {:?} -> {}", i, c, cuboid_size(c)));
    println!(
        "------------------ Size: {} --------------------------",
        count_cubes(reactor)
    );
}

/// Main function
pub fn main() {
    let steps = read_steps(include_str!("../../input/day22.txt"));
    let mut reactor = Reactor::new();
    for step in steps.iter() {
        apply_step(&mut reactor, step);
    }
    println!("Step 1: {}", reactor.len());

    // Ofcourse, for part 2 we need to completely revisit the algorithm. We should have known from
    // the "initialization procedure region" that this was foreshadowing a much larger problem in part 2.
    //
    // Because we only have to count the lit cubes, we can use a simple vector of cuboids, and just add
    // the volume of each cuboid to the total.
    //
    // If the cuboids would not overlap, this would work. But they do, and we have 'subtracting' steps...
    // What we need to do is only add the 'positive' steps, not the negative steps.
    // But for both steps, we should add 'negative' cuboids to account for overlap correction and partial
    // cube 'off' results.
    // This must work.
    let mut reactor = Reactor2::new();
    for step in steps {
        apply_step2(&mut reactor, &step);
    }
    println!("Step 2: {}", count_cubes(&reactor));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_read_steps() {
        let steps = read_steps(
            r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#,
        );
        assert_eq!(steps.len(), 4);

        let step = &steps[0];
        assert!(step.value);
        assert_eq!(step.cuboid, ((10, 13), (10, 13), (10, 13)));

        let step = &steps[1];
        assert!(step.value);
        assert_eq!(step.cuboid, ((11, 14), (11, 14), (11, 14)));

        let step = &steps[2];
        assert!(!step.value);
        assert_eq!(step.cuboid, ((9, 12), (9, 12), (9, 12)));

        let step = &steps[3];
        assert!(step.value);
        assert_eq!(step.cuboid, ((10, 11), (10, 11), (10, 11)));
    }

    #[test]
    fn test_reactor() {
        let steps = read_steps(
            r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#,
        );

        // create a new reactor
        let mut reactor = Reactor::new();

        // apply the first step
        apply_step(&mut reactor, &steps[0]);

        // now the reactor should have 27 cubes lit
        assert_eq!(reactor.len(), 27);

        // apply the second step
        apply_step(&mut reactor, &steps[1]);

        // now an additional 19 cubes are lit
        assert_eq!(reactor.len(), 27 + 19);

        // apply the third step
        apply_step(&mut reactor, &steps[2]);

        // 8 cubes are now off, so there are now 27 + 19 - 8 = 38 cubes lit
        assert_eq!(reactor.len(), 38);

        // apply the fourth step
        apply_step(&mut reactor, &steps[3]);

        // this only lit one cube
        assert_eq!(reactor.len(), 39);
    }

    #[test]
    fn test_in_working_range() {
        let step = Step {
            value: true,
            cuboid: ((10, 12), (10, 12), (10, 12)),
        };
        assert!(step.in_working_range());

        let step = Step {
            value: true,
            cuboid: ((-11, -13), (11, 13), (11, 13)),
        };
        assert!(step.in_working_range());

        let step = Step {
            value: true,
            cuboid: ((-11, -13), (111, 113), (11, 13)),
        };
        assert!(!step.in_working_range());
    }

    #[test]
    fn test_larger_example() {
        let steps = read_steps(include_str!("../../input/day22_ex.txt"));

        // create a reactor
        let mut reactor = Reactor::new();

        // apply the steps
        for step in &steps {
            apply_step(&mut reactor, step);
        }

        // there should be 590784 cubes lit
        assert_eq!(reactor.len(), 590784);
    }

    #[test]
    fn test_cuboid_size() {
        let cuboid = ((0, 1), (0, 1), (0, 1));
        assert_eq!(cuboid_size(&cuboid), 1);

        let cuboid = ((1, 0), (1, 0), (1, 0));
        assert_eq!(cuboid_size(&cuboid), -1);
    }

    #[test]
    fn test_overlap() {
        assert_eq!(overlap((0, 6), (3, 7)), Some((3, 6)));
        assert_eq!(overlap((0, 2), (2, 3)), None);
    }

    #[test]
    fn test_reactor2() {
        let mut reactor = Reactor2::new();

        // add a step of a cuboid that is 5x5x5
        let step = Step {
            value: true,
            cuboid: ((0, 5), (0, 5), (0, 5)),
        };

        // apply the step
        apply_step2(&mut reactor, &step);

        // there should be 5^3 = 125 cubes lit
        assert_eq!(count_cubes(&reactor), 125);

        // add a step of a cuboid that is 5x5x5, and starts at (0, 1, 0)
        let step = Step {
            value: true,
            cuboid: ((0, 5), (1, 6), (0, 5)),
        };

        // apply the step
        apply_step2(&mut reactor, &step);

        // because the 5x5x5 cuboid was shifted only 1 cube over, the total number of lit cubes should be 130
        assert_eq!(count_cubes(&reactor), 150);
    }

    #[test]
    fn test_part2_step_consolidation() {
        let steps = read_steps(
            r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#,
        );

        // since we consider the representation of what is lit to consist of cuboids of "on" state,
        // our reactor is simply a list of cuboids.
        let mut reactor = Reactor2::new();

        // apply the first step
        apply_step2(&mut reactor, &steps[0]);
        // print what's in the reactor
        print_reactor2(&reactor);

        assert_eq!(count_cubes(&reactor), 27);

        // apply the second step
        apply_step2(&mut reactor, &steps[1]);

        // print what's in the reactor
        print_reactor2(&reactor);

        assert_eq!(count_cubes(&reactor), 27 + 19);

        // apply the third step
        apply_step2(&mut reactor, &steps[2]);

        // print what's in the reactor
        print_reactor2(&reactor);

        assert_eq!(count_cubes(&reactor), 38);

        // apply the fourth step
        apply_step2(&mut reactor, &steps[3]);

        // we should now have 39 cubes lit
        assert_eq!(count_cubes(&reactor), 39);
    }

    #[test]
    fn test_larger_example_part2() {
        let steps = read_steps(include_str!("../../input/day22_ex2.txt"));

        // create a reactor
        let mut reactor = Reactor2::new();

        // apply the steps
        for step in &steps {
            apply_step2(&mut reactor, step);
        }

        assert_eq!(count_cubes(&reactor), 2758514936282235);
    }
}
