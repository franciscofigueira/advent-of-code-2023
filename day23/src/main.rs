use std::{collections::HashMap, fs::File, io::Read, thread, vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    row: usize,
    col: usize,
    previous_positions: Vec<u64>,
}

impl State {
    fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(Self, u32)> {
        if self.row == grid.len() - 1 && self.col == grid[0].len() - 2 {
            return vec![];
        }
        let mut successors: Vec<(Self, u32)> = Vec::new();
        let col = self.col;
        let row = self.row;
        let mut previous_positions = self.previous_positions.clone();
        let n_cols = grid[0].len();
        set_bit(&mut previous_positions, row, col, n_cols);

        let current_char = grid[row][col];
        if current_char == '>' {
            if get_bit(&previous_positions, row, col + 1, n_cols) {
                return vec![];
            }
            return vec![(
                Self {
                    row,
                    col: col + 1,
                    previous_positions,
                },
                1,
            )];
        } else if current_char == '<' {
            if get_bit(&previous_positions, row, col - 1, n_cols) {
                return vec![];
            }
            return vec![(
                Self {
                    row,
                    col: col - 1,
                    previous_positions,
                },
                1,
            )];
        } else if current_char == 'v' {
            if get_bit(&previous_positions, row + 1, col, n_cols) {
                return vec![];
            }
            return vec![(
                Self {
                    row: row + 1,
                    col,
                    previous_positions,
                },
                1,
            )];
        } else if current_char == '^' {
            if get_bit(&previous_positions, row - 1, col, n_cols) {
                return vec![];
            }
            return vec![(
                Self {
                    row: row - 1,
                    col,
                    previous_positions,
                },
                1,
            )];
        } else {
            //check up
            if row != 0
                && grid[row - 1][col] != '#'
                && !get_bit(&previous_positions, row - 1, col, n_cols)
            {
                successors.push((
                    Self {
                        row: row - 1,
                        col,
                        previous_positions: previous_positions.clone(),
                    },
                    1,
                ))
            }
            //check down
            if grid[row + 1][col] != '#' && !get_bit(&previous_positions, row + 1, col, n_cols) {
                successors.push((
                    Self {
                        row: row + 1,
                        col,
                        previous_positions: previous_positions.clone(),
                    },
                    1,
                ))
            }
            //check right
            if col != grid[0].len() - 1
                && grid[row][col + 1] != '#'
                && !get_bit(&previous_positions, row, col + 1, n_cols)
            {
                successors.push((
                    Self {
                        row,
                        col: col + 1,
                        previous_positions: previous_positions.clone(),
                    },
                    1,
                ))
            }
            //check left
            if col != 0
                && grid[row][col - 1] != '#'
                && !get_bit(&previous_positions, row, col - 1, n_cols)
            {
                successors.push((
                    Self {
                        row,
                        col: col - 1,
                        previous_positions: previous_positions.clone(),
                    },
                    1,
                ))
            }
        }
        successors
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State2 {
    index: usize,
    prev_pos: Vec<u64>,
}
impl State2 {
    fn get_successors(
        &self,
        graph: &HashMap<usize, Vec<(usize, usize)>>,
        goal: usize,
    ) -> Vec<(Self, usize)> {
        if self.index == goal {
            return vec![];
        }
        let mut successors: Vec<(Self, usize)> = Vec::new();

        let mut cl = self.prev_pos.clone();
        set_bit2(&mut cl, self.index);
        for (next_node, weight) in graph.get(&self.index).unwrap() {
            if !get_bit2(&cl, next_node) {
                successors.push((
                    Self {
                        index: next_node.clone(),
                        prev_pos: cl.clone(),
                    },
                    weight.clone(),
                ));
            }
        }

        successors
    }
}
fn set_bit2(words: &mut Vec<u64>, index: usize) {
    let word_pos = index / 64;
    let bit_pos = index % 64;
    let mask = 1 << bit_pos;
    let word = words.get_mut(word_pos as usize).unwrap();
    *word ^= mask;
}
fn get_bit2(words: &Vec<u64>, index: &usize) -> bool {
    let word_pos = index / 64;
    let bit_pos = index % 64;
    let mask = 1 << bit_pos;
    let word = words.get(word_pos as usize).unwrap();
    word & mask != 0
}

fn get_node_index(row: usize, col: usize, n_cols: usize) -> u64 {
    (row * n_cols + col) as u64
}
fn get_bit_pos(row: usize, col: usize, n_cols: usize) -> (u64, u64) {
    let index = get_node_index(row, col, n_cols);
    let word_pos = index / 64;
    let bit_pos = index % 64;
    return (word_pos, bit_pos);
}
fn set_bit(words: &mut Vec<u64>, row: usize, col: usize, n_cols: usize) {
    let (word_pos, bit_pos) = get_bit_pos(row, col, n_cols);
    let mask = 1 << bit_pos;
    let word = words.get_mut(word_pos as usize).unwrap();
    *word ^= mask;
}
fn get_bit(words: &Vec<u64>, row: usize, col: usize, n_cols: usize) -> bool {
    let (word_pos, bit_pos) = get_bit_pos(row, col, n_cols);
    let mask = 1 << bit_pos;
    let word = words.get(word_pos as usize).unwrap();
    word & mask != 0
}

fn main() {
    let mut file = File::open("./test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let bool_grid = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|cha| {
                    if cha == &'#' {
                        return false;
                    } else {
                        return true;
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    let n_cols = grid[0].len();
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let goal = (grid.len() - 1) * n_cols + (grid[0].len() - 2);

    let start = 1;
    for i in 0..bool_grid.len() {
        for j in 0..bool_grid[i].len() {
            if bool_grid[i][j] {
                let index = i * n_cols + j;
                if index == goal {
                    continue;
                }
                if index == start {
                    graph.insert(start, vec![(n_cols + 1, 1)]);
                    continue;
                }
                let mut edges: Vec<(usize, usize)> = Vec::new();
                if i != 0 && bool_grid[i - 1][j] {
                    let mut k = i - 1;
                    loop {
                        if k != 0 && !bool_grid[k][j + 1] && !bool_grid[k][j - 1] && bool_grid[k][j]
                        {
                            k -= 1;
                        } else {
                            edges.push((k * n_cols + j, i - k));
                            break;
                        }
                    }
                }

                if bool_grid[i + 1][j] {
                    let mut k = i + 1;
                    loop {
                        if k != grid.len() - 1
                            && !bool_grid[k][j + 1]
                            && !bool_grid[k][j - 1]
                            && bool_grid[k][j]
                        {
                            k += 1;
                        } else {
                            edges.push((k * n_cols + j, k - i));
                            break;
                        }
                    }
                }
                if bool_grid[i][j - 1] {
                    let mut k = j - 1;
                    loop {
                        if k != 0 && !bool_grid[i + 1][k] && !bool_grid[i - 1][k] && bool_grid[i][k]
                        {
                            k -= 1;
                        } else {
                            edges.push((i * n_cols + k, j - k));
                            break;
                        }
                    }
                }
                if bool_grid[i][j + 1] {
                    let mut k = j + 1;
                    loop {
                        if k != grid[0].len() - 1
                            && !bool_grid[i + 1][k]
                            && !bool_grid[i - 1][k]
                            && bool_grid[i][k]
                        {
                            k += 1;
                        } else {
                            edges.push((i * n_cols + k, k - j));
                            break;
                        }
                    }
                }
                graph.insert(index, edges);
            }
        }
    }
    println!("{graph:?}");

    let num_words = ((grid.len() * grid[0].len()) / 64) + 1;

    let initial_state = State {
        row: 0,
        col: 1,
        previous_positions: vec![0; num_words],
    };
    let r = part_1(&initial_state, &grid, grid.len() - 1, grid[0].len() - 2);
    let part_1_res = r[r.len() - 1];
    println!("part 1 res {part_1_res}");

    let initial_state2 = State2 {
        index: 1,
        prev_pos: vec![0; num_words],
    };

    let mut r2 = solve5(&initial_state2, &graph, goal);
    r2.sort();
    println!("{r2:?}");
    let part_2_res = r2[r2.len() - 1];
    println!("part 2 res {part_2_res}");
}

fn part_1(
    initial_state: &State,
    grid: &Vec<Vec<char>>,
    end_row: usize,
    end_col: usize,
) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();

    let mut states_left: Vec<(State, u32)> = Vec::new();
    states_left.push((initial_state.clone(), 1));

    while let Some((state, current_cost)) = states_left.pop() {
        let next_states = state.successors(&grid);
        for (next_state, _) in next_states {
            if next_state.row == end_row && next_state.col == end_col {
                results.push(current_cost);
            } else {
                states_left.push((next_state, current_cost + 1));
            }
        }
    }
    results.sort();
    results
}

fn solve4(
    initial_state: &State2,
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    goal: usize,
) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    let mut states_left: Vec<(State2, u32)> = Vec::new();
    states_left.push((initial_state.clone(), 1));

    while let Some((state, current_cost)) = states_left.pop() {
        let next_states = state.get_successors(&graph, goal);
        //println!("{next_states:?}");
        for (next_state, weight) in next_states {
            if next_state.index == goal {
                results.push(current_cost);
                //println!("found {current_cost}");
                continue;
            }

            states_left.push((next_state, current_cost + weight as u32));
            //let len = states_left.len();
        }
    }

    results
}

fn solve5(
    initial_state: &State2,
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    goal: usize,
) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    let mut states_left: Vec<(State2, u32)> = Vec::new();
    states_left.push((initial_state.clone(), 1));

    'outer: while let Some((state, current_cost)) = states_left.pop() {
        let next_states = state.get_successors(&graph, goal);
        //println!("{next_states:?}");
        for (next_state, weight) in next_states {
            if next_state.index == goal {
                results.push(current_cost);
                //println!("found {current_cost}");
                continue;
            }
            if states_left.len() == 50 {
                break 'outer;
            }

            states_left.push((next_state, current_cost + weight as u32));
        }
    }

    let mut handlers = Vec::new();
    for state in states_left {
        let g_cl = graph.clone();
        let g_c = goal.clone();
        handlers.push(thread::spawn(move || {
            solve_helper2(state, g_cl, g_c.clone())
        }));
    }
    for handler in handlers {
        results.append(handler.join().expect("join").as_mut());
    }

    results
}

fn solve_helper2(
    state: (State2, u32),
    graph: HashMap<usize, Vec<(usize, usize)>>,
    goal: usize,
) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    let mut states_left: Vec<(State2, u32)> = Vec::new();
    states_left.push(state);

    while let Some((state, current_cost)) = states_left.pop() {
        let next_states = state.get_successors(&graph, goal);
        for (next_state, weight) in next_states {
            if next_state.index == goal {
                results.push(current_cost);
                //println!("found {current_cost}");
                continue;
            }

            states_left.push((next_state, current_cost + weight as u32));
        }
    }

    results
}

#[test]
fn test_bit_set() {
    let grid_len = 100;
    let n_cols = 100;
    let row = 10;
    let col = 12;
    let num_words = grid_len * n_cols / 64;
    let mut words = vec![0; num_words];

    assert!(!get_bit(&mut words, row, col, n_cols));
    set_bit(&mut words, row, col, n_cols);
    assert!(get_bit(&words, row, col, n_cols));
    assert!(!get_bit(&mut words, row + 1, col, n_cols));
    assert!(!get_bit(&mut words, 0, 0, n_cols));
    set_bit(&mut words, 0, 0, n_cols);
    assert!(get_bit(&mut words, 0, 0, n_cols));
}
