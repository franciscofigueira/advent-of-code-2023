use pathfinding::prelude::dijkstra_all;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    row: usize,
    col: usize,
    num_steps: usize,
}

impl State {
    fn get_successors(&self, grid: &Vec<Vec<char>>, max_num_steps: usize) -> Vec<(Self, u32)> {
        if self.num_steps >= max_num_steps {
            return vec![];
        }
        let mut successors: Vec<Self> = Vec::new();

        let col = self.col;
        let row = self.row;
        //move right
        if col + 1 < grid[0].len() && grid[row][col + 1] == '.' {
            successors.push(State {
                row,
                col: col + 1,
                num_steps: self.num_steps + 1,
            });
        }
        //move left
        if col != 0 && grid[row][col - 1] == '.' {
            successors.push(State {
                row,
                col: col - 1,
                num_steps: self.num_steps + 1,
            });
        }

        //move up

        if row != 0 && grid[row - 1][col] == '.' {
            successors.push(State {
                row: row - 1,
                col,
                num_steps: self.num_steps + 1,
            });
        }

        //move down
        if row != grid.len() - 1 && grid[row + 1][col] == '.' {
            successors.push(State {
                row: row + 1,
                col,
                num_steps: self.num_steps + 1,
            });
        }

        successors
            .iter()
            .map(|s| (s.clone(), 1))
            .collect::<Vec<_>>()
    }
}

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    let grid = contents
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut parsed = Vec::new();

            for (col, char) in line.chars().enumerate() {
                if char == 'S' {
                    start_row = row;
                    start_col = col;
                    parsed.push('.')
                } else {
                    parsed.push(char);
                }
            }
            parsed
        })
        .collect::<Vec<Vec<char>>>();

    let initial_state = State {
        row: start_row,
        col: start_col,
        num_steps: 0,
    };
    let part_1_res = part_1(&initial_state, &grid, 64);
    println!("part 1 res: {part_1_res}");
    let part_2_res = part_2(&initial_state, &grid, 66);
    println!("part 2 res: {part_2_res}");
}

fn part_1(initial_state: &State, grid: &Vec<Vec<char>>, num_steps: usize) -> usize {
    //https://docs.rs/pathfinding/latest/pathfinding/directed/dijkstra/fn.dijkstra_all.html
    let result = dijkstra_all(initial_state, |s| s.get_successors(grid, num_steps));
    let r = result
        .into_iter()
        .filter_map(|s| {
            if s.0.num_steps == num_steps {
                Some(())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    r.len()
}

//https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn part_2(initial_state: &State, grid: &Vec<Vec<char>>, num_steps: usize) -> usize {
    let mut result = dijkstra_all(initial_state, |s| s.get_successors(grid, num_steps));

    result.insert(initial_state.clone(), (initial_state.clone(), 0));
    let visited = result
        .into_iter()
        .map(|(node, (_, cost))| (node, cost))
        .collect::<HashMap<State, u32>>();
    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited.values().filter(|v| **v % 2 == 1).count();

    // This is 202300 but im writing it out here to show the process
    let n = 202300 as usize;

    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;
    p2
}
