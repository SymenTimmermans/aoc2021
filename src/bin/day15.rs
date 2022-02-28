use aoc2021::read_strs;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

/// Day 15 looks like a simple 'shortest-path' problem.
/// So lets just try to implement Dijkstra on this.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    node: usize,
    cost: usize,
}

fn file_to_grid(file_path: &str) -> Vec<Vec<usize>> {
    let lines = read_strs(file_path);

    // first, read the lines into a 2d grid of costs
    let mut grid = vec![vec![]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        grid[i] = vec![0; line.len()];
        for (j, c) in line.chars().enumerate() {
            // parse numeric char into int value
            let cost = c.to_digit(10).unwrap() as usize;

            grid[i][j] = cost;
        }
    }

    grid
}

fn grid_to_graph(grid: &[Vec<usize>]) -> Vec<Vec<Edge>> {
    let width = grid[0].len();
    let height = grid.len();

    // now, create the graph from looping through the grid
    let mut graph = vec![vec![]; width * height];
    for i in 0..height {
        for j in 0..width {
            let mut cell_costs = vec![];

            // get cellindex from i and j
            let cell_index = i * width + j;

            // if not on the top row, we can add the cost to move to the cell above
            if i > 0 {
                let above_index = (i - 1) * width + j;
                cell_costs.push(Edge {
                    node: above_index,
                    cost: grid[i - 1][j],
                });
            }

            // if not on the bottom row, we can add the cost to move to the cell below
            if i < height - 1 {
                let below_index = (i + 1) * width + j;
                cell_costs.push(Edge {
                    node: below_index,
                    cost: grid[i + 1][j],
                });
            }

            // if not on the left column, we can add the cost to move to the cell to the left
            if j > 0 {
                let left_index = i * width + j - 1;
                cell_costs.push(Edge {
                    node: left_index,
                    cost: grid[i][j - 1],
                });
            }

            // if not on the right column, we can add the cost to move to the cell to the right
            if j < width - 1 {
                let right_index = i * width + j + 1;
                cell_costs.push(Edge {
                    node: right_index,
                    cost: grid[i][j + 1],
                });
            }

            graph[cell_index] = cell_costs;
        }
    }

    graph
}

pub fn read_graph(file_path: &str) -> Vec<Vec<Edge>> {
    let grid = file_to_grid(file_path);
    grid_to_graph(&grid)
}

pub fn read_graph_p2(file_path: &str) -> Vec<Vec<Edge>> {
    let grid = file_to_grid(file_path);
    let grid = extrapolate(&grid);
    grid_to_graph(&grid)
}

/// This function takes in a grid, and extrapolates it five times
/// to the right and to the down.
/// Every cell is then copied to the reflected cell.
/// The value of the cell is wrapped to one if it is higher than 9.
fn extrapolate(grid: &[Vec<usize>]) -> Vec<Vec<usize>> {
    // first we need to get the width and height of the grid
    let width = grid[0].len();
    let height = grid.len();

    // now we need to create a new grid that is 5 times bigger
    let mut new_grid = vec![vec![0; width * 5]; height * 5];

    // now we can copy the values from the old grid to the new grid
    for i in 0..height {
        for j in 0..width {
            // for every original cell, it is copied 5*5 times to the new grid
            for k in 0..5 {
                for l in 0..5 {
                    let mut new_value = grid[i][j] + k + l;
                    while new_value > 9 {
                        new_value -= 9;
                    }
                    new_grid[i + (k * height)][j + (l * width)] = new_value;
                }
            }
        }
    }

    new_grid
}

pub fn main() {
    let graph = read_graph("input/day15.txt");
    let bottom_right = graph.len() - 1;

    let result = shortest_path(&graph, 0, bottom_right);
    println!("Part 1: {:?}", result);

    // Part 2:
    let biggraph = read_graph_p2("input/day15.txt");
    let bottom_right = biggraph.len() - 1;
    let result = shortest_path(&biggraph, 0, bottom_right);

    println!("Part 2: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![
                Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 },
            ],
            // Node 3
            vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
            // Node 4
            vec![],
        ];

        assert_eq!(shortest_path(&graph, 0, 1), Some(1));
        assert_eq!(shortest_path(&graph, 0, 3), Some(3));
        assert_eq!(shortest_path(&graph, 3, 0), Some(7));
        assert_eq!(shortest_path(&graph, 0, 4), Some(5));
        assert_eq!(shortest_path(&graph, 4, 0), None);
    }

    #[test]
    fn test_example() {
        let graph = read_graph("input/day15_ex.txt");

        // assert that the graph has 100 elements
        assert_eq!(graph.len(), 100);

        assert_eq!(shortest_path(&graph, 0, 1), Some(1));
        assert_eq!(shortest_path(&graph, 0, 99), Some(40));
    }

    #[test]
    fn test_extrapolation() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = extrapolate(&grid);

        // assert that the graph has 3 * 3 by 5 * 5 elements
        assert_eq!(grid.len(), 3 * 5);
        assert_eq!(grid[0].len(), 3 * 5);

        // test the wrapping of the values
        assert_eq!(grid[0][0], 1);
        assert_eq!(grid[0][3], 2);
        assert_eq!(grid[0][6], 3);

        assert_eq!(grid[3][0], 2);
        assert_eq!(grid[6][0], 3);

        assert_eq!(grid[2][2], 9);
        assert_eq!(grid[5][2], 1);
        assert_eq!(grid[2][5], 1);
        assert_eq!(grid[5][5], 2);
        assert_eq!(grid[8][8], 4);
    }

    #[test]
    fn test_example_p2() {
        let graph = read_graph_p2("input/day15_ex.txt");

        // assert that the graph has 2500 elements
        assert_eq!(graph.len(), 2500);

        // original shortest paths should still work
        assert_eq!(shortest_path(&graph, 0, 1), Some(1));
        assert_eq!(shortest_path(&graph, 0, 9 * 50 + 9), Some(40));

        // extrapolated shortest paths should work
        assert_eq!(shortest_path(&graph, 0, 2499), Some(315));
    }
}
