use std::{fs::File, io::Read};

use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    col: usize,
    row: usize,
    direction: Direction,
    streak: u32,
}

impl State {
    fn get_successors(
        &self,
        grid: &Vec<Vec<u32>>,
        min_streak: u32,
        max_streak: u32,
    ) -> Vec<(Self, u32)> {
        let mut successors: Vec<_> = Vec::new();

        let row = self.row;
        let col = self.col;

        match self.direction {
            Direction::Up => {
                //go straight
                if self.streak < max_streak && row != 0 {
                    successors.push(State {
                        col,
                        row: row - 1,
                        direction: Direction::Up,
                        streak: self.streak + 1,
                    })
                }
                //go right
                if col != grid[0].len() - 1 && self.streak >= min_streak {
                    successors.push(State {
                        col: col + 1,
                        row,
                        direction: Direction::Right,
                        streak: 1,
                    })
                }

                //go left
                if col != 0 && self.streak >= min_streak {
                    successors.push(State {
                        col: col - 1,
                        row,
                        direction: Direction::Left,
                        streak: 1,
                    })
                }
            }
            Direction::Down => {
                //go straight
                if self.streak < max_streak && row != grid.len() - 1 {
                    successors.push(State {
                        col,
                        row: row + 1,
                        direction: Direction::Down,
                        streak: self.streak + 1,
                    })
                }
                //go right
                if col != grid[0].len() - 1 && self.streak >= min_streak {
                    successors.push(State {
                        col: col + 1,
                        row,
                        direction: Direction::Right,
                        streak: 1,
                    })
                }

                //go left
                if col != 0 && self.streak >= min_streak {
                    successors.push(State {
                        col: col - 1,
                        row,
                        direction: Direction::Left,
                        streak: 1,
                    })
                }
            }
            Direction::Right => {
                //go straight
                if self.streak < max_streak && col < grid[0].len() - 1 {
                    successors.push(State {
                        col: col + 1,
                        row,
                        direction: Direction::Right,
                        streak: self.streak + 1,
                    })
                }
                //go up
                if row != 0 && self.streak >= min_streak {
                    successors.push(State {
                        col,
                        row: row - 1,
                        direction: Direction::Up,
                        streak: 1,
                    })
                }

                //go down
                if row != grid.len() - 1 && self.streak >= min_streak {
                    successors.push(State {
                        col,
                        row: row + 1,
                        direction: Direction::Down,
                        streak: 1,
                    })
                }
            }
            Direction::Left => {
                //go straight
                if self.streak < max_streak && col != 0 {
                    successors.push(State {
                        col: col - 1,
                        row,
                        direction: Direction::Left,
                        streak: self.streak + 1,
                    })
                }
                //go up
                if row != 0 && self.streak >= min_streak {
                    successors.push(State {
                        col,
                        row: row - 1,
                        direction: Direction::Up,
                        streak: 1,
                    })
                }

                //go down
                if row != grid.len() - 1 && self.streak >= min_streak {
                    successors.push(State {
                        col,
                        row: row + 1,
                        direction: Direction::Down,
                        streak: 1,
                    })
                }
            }
        }

        let successors = successors
            .iter()
            .map(|state| (state.clone(), grid[state.row][state.col]))
            .collect::<Vec<_>>();
        successors
    }
}

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let grid = contents
        .lines()
        .map(|line| {
            let mut parsed: Vec<u32> = Vec::new();
            for char in line.chars() {
                parsed.push(char.to_digit(10).unwrap());
            }
            parsed
        })
        .collect::<Vec<Vec<u32>>>();

    let initial_state = State {
        col: 0,
        row: 0,
        direction: Direction::Down,
        streak: 0,
    };

    let part_1_res = solve(&initial_state, &grid, 0, 3);
    println!("part 1 res: {part_1_res}");
    let part_2_res = solve(&initial_state, &grid, 4, 10);
    println!("part 2 res: {part_2_res}");
}

fn solve(initial_state: &State, grid: &Vec<Vec<u32>>, min_streak: u32, max_streak: u32) -> u32 {
    let result = dijkstra(
        initial_state,
        |state| state.get_successors(&grid, min_streak, max_streak),
        |n| n.row == grid.len() - 1 && n.col == grid[0].len() - 1 && n.streak >= min_streak,
    );

    return result.unwrap().1;
}
