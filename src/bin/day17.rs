/// This task is a simulation task. How high can we shoot a probe while still landing
/// it on the target area.
///

/// This struct Vec2 can be used for positions and velocities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

/// The probe has a position and a velocity.
/// It also knows its target area and max_y
struct Probe {
    pos: Vec2,
    vel: Vec2,
    target: (Vec2, Vec2),
    max_y: i32,
}

impl Probe {
    /// Create a new probe.
    fn new(vel: Vec2, tx1: i32, tx2: i32, ty1: i32, ty2: i32) -> Probe {
        Probe {
            pos: Vec2 { x: 0, y: 0 },
            vel,
            target: (Vec2 { x: tx1, y: ty1 }, Vec2 { x: tx2, y: ty2 }),
            max_y: 0,
        }
    }

    /// Move the probe one step.
    fn step(&mut self) {
        // The probe's x position increases by its x velocity.
        self.pos.x += self.vel.x;
        // The probe's y position increases by its y velocity.
        self.pos.y += self.vel.y;
        // Due to drag, the probe's x velocity changes by 1 toward the value 0;
        // that is, it decreases by 1 if it is greater than 0, increases by 1
        // if it is less than 0, or does not change if it is already 0.
        match self.vel.x {
            x if x > 0 => self.vel.x -= 1,
            x if x < 0 => self.vel.x += 1,
            _ => (),
        }
        // Due to gravity, the probe's y velocity decreases by 1.
        self.vel.y -= 1;

        // update max_y
        if self.pos.y > self.max_y {
            self.max_y = self.pos.y;
        }
    }

    /// Check if the probe has landed on the target area.
    fn on_target(&self) -> bool {
        self.pos.x >= self.target.0.x
            && self.pos.x <= self.target.1.x
            && self.pos.y >= self.target.0.y
            && self.pos.y <= self.target.1.y
    }

    /// Checks if the probe will land on target area after x steps,
    /// and returns the max_y.
    fn reaches_target(&mut self) -> Option<i32> {
        while !self.on_target() {
            self.step();
            // if the x position is to the right of the target area,
            // the probe will not reach the target area.
            if self.pos.x > self.target.1.x {
                return None;
            }
            // if the y position is below the target area,
            // and the y velocity is negative,
            // the probe will not reach the target area.
            if self.pos.y < self.target.0.y && self.vel.y < 0 {
                return None;
            }
        }
        Some(self.max_y)
    }
}

/// Quite possibly, finding the best solution that gives the highest max_y is not a
/// matter of iteratively trying all possible solutions.
///
/// We should probably deduct some boundaries based on the rules of the movement.
/// Our initial velocity is modified by the drag and gravity.
///
/// Luckily, the x and y parts of the velocity are modified by different rules, so we
/// can separate the deduction into x and y deduction.
///
/// We need at least enough x velocity to reach the target area, considering the drag.
/// We need at least enough y velocity to reach the target area, considering the gravity.
///
/// Maybe we can reverse the algorithm, and try to find all the velocities that will
/// get the probe to a given coordinate. At least this prevents us from simulating
/// trajectories that will never reach the target area. If we found all valid starting
/// velocities for the entire target area, we can keep only the ones with the highest y
/// velocities. That will make the problem a lot smaller.
///
/// Looking at the trajectory, there is an important property for x.
/// Steps in x direction decrease by 1 every time, until they hit 0
/// So for instance, to get to x 11, start velocity can only be either:
/// 11, or 6(+5), reaching it respectively in 1 or 2 steps.
/// To get to x 10, start velocity can only be either:
/// 10, or 4(+3+2+1), reaching it respectively in 1 or 4 steps.
/// This property allows us to determine a finite list of starting x velocities for
/// a desired x coordinate
///
/// Just because we can, lets create a function that allows to pass lower and upper x
/// bounds, because the checking is the simple part of the algorithm.
pub fn find_x_vels(x1: i32, x2: i32) -> Vec<i32> {
    let mut vels = Vec::new();

    // if zero is a solution, add it now, so we can exclude it from the iteration
    if 0 >= x1 && 0 <= x2 {
        vels.push(0);
    }

    // take the upper bound as max x velocity. Any higher and we will overshoot.
    // we can work downwards to 1.
    for test_x in (1..=x2).rev() {
        // initialize travelled distance to test_x
        let mut travel = test_x;
        let mut step_size = test_x;
        // as long as we don't overshoot, we can do steps.
        while travel <= x2 && step_size > 0 {
            // test if we are in the target area
            if travel >= x1 {
                // we are, add the velocity to the list
                vels.push(test_x);
            }
            // we're not in the target area, so we can do a step.
            // decrease step size and add it to travel
            step_size -= 1;
            travel += step_size;
        }
    }
    vels.dedup();
    vels
}

/// Ofcourse we can also employ this trick for y. It's a little more complicated, because
/// the Y velocity goes up and down again.
/// There's a little bit of help, because the probe goes up and down hitting the same y
/// values. So we can assume that somewhere in the trajectory, the y value
/// will be the same as the starting value (0). The velocity downwards will be one more than
/// what it started with.
/// We can use this property to our advantage. If we can figure out the starting
/// velocity from the zero axis towards the target area, we also know that we can subtract
/// one from that starting velocity to know what the upwards velocity should be to
/// end up there.
/// for instance, if we start at y=0, and we want to end up at y=10
/// we can give it a y velocity of -10, so after one step, the probe will be at -10.
/// if we give it a velocity of -9, it will fall short, and the next step, it will
/// overshoot, because the velocity decreases to -10, and the probe will be at -19.
/// However, if we give it a starting velocity of -1, it will end up at -10, look:
/// vel -1, pos -1
/// vel -2, pos -3
/// vel -3, pos -6
/// vel -4, pos -10 - yay! So we should return -10 and -1 as possible solutions,
/// but also +9 and 0 are possible solutions.
pub fn find_y_vels(y1: i32, y2: i32) -> Vec<i32> {
    let mut vels = Vec::new();

    // if zero is a solution, add it now, so we can exclude it from the iteration
    if 0 >= y1 && 0 <= y2 {
        vels.push(0);
    }

    // take the upper bound as max y velocity. Any higher and we will overshoot.
    // we can work downwards to 1.
    for test_y in y1..=-1 {
        // initialize travelled distance to test_y
        let mut travel = test_y;
        let mut step_size = -test_y;

        // as long as we don't overshoot, we can do steps.
        while travel >= y1 {
            // test if we are in the target area
            if travel <= y2 {
                // we are, add the velocity to the list
                vels.push(test_y);
                // also add the upwards velocity, which is flipped, minus one
                vels.push(-test_y - 1);
            }

            // we're not in the target area, so we can do a step.
            // increase step size and add it to travel
            step_size += 1;
            travel -= step_size;
        }
    }
    vels.sort_unstable();
    vels.reverse();
    vels.dedup();
    vels
}

/// Now that we have valid x and y velocities, it does not autmatically mean that every
/// combination of velocities will be a valid solution. Since some y velocities will hit
/// after one step, some x velocities will need more steps to reach the target area.
/// However, since we have a very finite list of combinations, we can simply check
/// every combination and report back on the highest y position, which is what we were
/// looking for initially.
pub fn find_max_y(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let x_vels = find_x_vels(x1, x2);
    let y_vels = find_y_vels(y1, y2);
    let mut max_y = 0;

    for x_vel in &x_vels {
        for y_vel in &y_vels {
            let mut probe = Probe::new(
                Vec2 {
                    x: *x_vel,
                    y: *y_vel,
                },
                x1,
                x2,
                y1,
                y2,
            );
            if let Some(my) = probe.reaches_target() {
                if my > max_y {
                    max_y = my;
                }
            }
        }
    }

    max_y
}

/// For step two, we are glad we took the effort to deduct a way to get the distinct
/// velocities for each axis that reach the target area.
/// The assignment of part two is to simply count the number of pairs that work
fn count_valid_values(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let x_vels = find_x_vels(x1, x2);
    let y_vels = find_y_vels(y1, y2);
    let mut count = 0;

    for x_vel in &x_vels {
        for y_vel in &y_vels {
            let mut probe = Probe::new(
                Vec2 {
                    x: *x_vel,
                    y: *y_vel,
                },
                x1,
                x2,
                y1,
                y2,
            );
            if probe.reaches_target().is_some() {
                count += 1;
            }
        }
    }
    count
}

pub fn main() {
    let max_y = find_max_y(257, 286, -101, -57);
    println!("max_y: {}", max_y);
    let valid_values = count_valid_values(257, 286, -101, -57);
    println!("valid_values: {}", valid_values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test the creation of a probe.
    fn test_probe_new() {
        let probe = Probe::new(Vec2 { x: 1, y: 2 }, 3, 4, 5, 6);
        assert_eq!(probe.pos, Vec2 { x: 0, y: 0 });
        assert_eq!(probe.vel, Vec2 { x: 1, y: 2 });
        assert_eq!(probe.target, (Vec2 { x: 3, y: 5 }, Vec2 { x: 4, y: 6 }));
    }

    #[test]
    /// test the movement of a probe.
    /// The probe's x position increases by its x velocity.  
    /// The probe's y position increases by its y velocity.
    /// Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    /// Due to gravity, the probe's y velocity decreases by 1.
    fn test_probe_step() {
        let mut probe = Probe::new(Vec2 { x: 7, y: 2 }, 20, 30, -10, -5);
        // do 7 steps
        for _s in 1..=7 {
            probe.step();
        }

        // probe should be on target!
        assert!(probe.on_target());
    }

    #[test]
    /// test the movement of a probe, and check if it reaches target after any steps.
    fn test_probe_reaches_target() {
        let mut probe = Probe::new(Vec2 { x: 6, y: 3 }, 20, 30, -10, -5);
        // if we call reaches_target(), we should get Some(max_y)
        let outcome = probe.reaches_target();
        // outcome should not be None
        assert_ne!(outcome, None);
    }

    #[test]
    /// test the movement of a probe, and check if it reaches target after any steps.
    fn test_probe_reaches_target_2() {
        let mut probe = Probe::new(Vec2 { x: 9, y: 0 }, 20, 30, -10, -5);
        // if we call reaches_target(), we should get None
        let outcome = probe.reaches_target();
        // outcome should not be None
        assert_ne!(outcome, None);
    }

    #[test]
    /// the probe should not reach the target if start velocity is 17, -4
    fn test_probe_not_reaches_target() {
        let mut probe = Probe::new(Vec2 { x: 17, y: -4 }, 20, 30, -10, -5);
        // if we call reaches_target(), we should get None
        let outcome = probe.reaches_target();
        // outcome should be None
        assert_eq!(outcome, None);
    }

    #[test]
    /// the probe should reach a max_y of 45 when start velocity is 6, 9
    fn test_probe_reaches_max_y() {
        let mut probe = Probe::new(Vec2 { x: 6, y: 9 }, 20, 30, -10, -5);
        // if we call reaches_target(), we should get Some(max_y)
        let outcome = probe.reaches_target();
        // outcome should be Some(45)
        assert_eq!(outcome, Some(45));
    }

    #[test]
    /// test find_x_vels
    fn test_find_x_vels() {
        assert_eq!(find_x_vels(10, 10), vec![10, 4]);
        assert_eq!(find_x_vels(11, 11), vec![11, 6]);
        assert_eq!(find_x_vels(10, 11), vec![11, 10, 6, 4]);
    }

    #[test]
    /// test find_y_vels
    fn test_find_y_vels() {
        assert_eq!(find_y_vels(-10, -10), vec![9, 0, -1, -10]);
        assert_eq!(find_y_vels(-11, -11), vec![10, 4, -5, -11]);
        assert_eq!(find_y_vels(-11, -10), vec![10, 9, 4, 0, -1, -5, -10, -11]);
    }

    #[test]
    /// test to find the working velocities in the find_vels functions
    fn test_find_vels() {
        let xvels = find_x_vels(20, 30);
        let yvels = find_y_vels(-10, -5);

        assert!(xvels.contains(&7));
        assert!(yvels.contains(&2));

        assert!(xvels.contains(&6));
        assert!(yvels.contains(&3));

        assert!(xvels.contains(&9));
        assert!(yvels.contains(&0));

        // test that x = 17 is not a solution, because we know it overshoots.
        assert!(!xvels.contains(&17));
    }

    #[test]
    /// we can now test if for the example area, the max_y is 45
    fn test_find_max_y() {
        assert_eq!(find_max_y(20, 30, -10, -5), 45);
    }
}
