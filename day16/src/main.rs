use std::{collections::HashMap, io::Read};

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    fn direction(&self) -> (isize, isize) {
        match *self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::RIGHT => (0, 1),
            Direction::LEFT => (0, -1),
        }
    }
}

const INITIAL_DIRECTION: Direction = Direction::RIGHT;
const INITIAL_POSITION: (isize, isize) = (0, 0);

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<Vec<char>> = contents
        .split("\r\n")
        .map(|x| x.chars().collect())
        .collect();

    let part_1_res = part_1(&lines);
    println!("part 1 result {part_1_res:?}");
    let part_2_res = part_2(&lines);
    println!("part 2 result {part_2_res:?}");
}

fn part_1(lines: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    let mut light_positions: HashMap<(isize, isize), bool> = HashMap::new();

    let mut light_history: HashMap<((isize, isize), Direction), bool> = HashMap::new();

    let mut light_beams: Vec<((isize, isize), Direction)> = Vec::new();
    light_beams.push((INITIAL_POSITION, INITIAL_DIRECTION));

    let mut rules: HashMap<(char, Direction), (Direction, Option<Direction>)> = HashMap::new();
    rules.insert(('|', Direction::UP), (Direction::UP, None));
    rules.insert(('|', Direction::DOWN), (Direction::DOWN, None));
    rules.insert(
        ('|', Direction::LEFT),
        (Direction::UP, Some(Direction::DOWN)),
    );
    rules.insert(
        ('|', Direction::RIGHT),
        (Direction::UP, Some(Direction::DOWN)),
    );
    rules.insert(('-', Direction::RIGHT), (Direction::RIGHT, None));
    rules.insert(('-', Direction::LEFT), (Direction::LEFT, None));
    rules.insert(
        ('-', Direction::UP),
        (Direction::RIGHT, Some(Direction::LEFT)),
    );
    rules.insert(
        ('-', Direction::DOWN),
        (Direction::LEFT, Some(Direction::RIGHT)),
    );
    rules.insert(('/', Direction::UP), (Direction::RIGHT, None));
    rules.insert(('/', Direction::DOWN), (Direction::LEFT, None));
    rules.insert(('/', Direction::RIGHT), (Direction::UP, None));
    rules.insert(('/', Direction::LEFT), (Direction::DOWN, None));

    rules.insert(('\\', Direction::UP), (Direction::LEFT, None));
    rules.insert(('\\', Direction::DOWN), (Direction::RIGHT, None));
    rules.insert(('\\', Direction::RIGHT), (Direction::DOWN, None));
    rules.insert(('\\', Direction::LEFT), (Direction::UP, None));

    let initial_symbol = lines[INITIAL_POSITION.0 as usize][INITIAL_POSITION.1 as usize];
    light_positions.insert(INITIAL_POSITION, true);
    light_history.insert((INITIAL_POSITION, INITIAL_DIRECTION), true);
    if initial_symbol != '.' {
        let next_step = rules
            .get(&(initial_symbol, INITIAL_DIRECTION))
            .expect("always have");
        if next_step.1.is_some() {
            light_beams[0].1 = next_step.0;
            light_beams.push((light_beams[0].0, next_step.1.expect("checked for Some")));
        } else {
            light_beams[0].1 = next_step.0;
        }
    }

    let height = lines.len();
    let length = lines[0].len();
    while light_beams.len() != 0 {
        for j in 0..light_beams.len() {
            if j >= light_beams.len() {
                continue;
            }
            // println!("{light_beams:?}");
            light_beams[j].0 .0 += light_beams[j].1.direction().0;
            light_beams[j].0 .1 += light_beams[j].1.direction().1;

            if light_beams[j].0 .0 < 0 || light_beams[j].0 .1 < 0 {
                light_beams.swap_remove(j);
                continue;
            }

            if light_beams[j].0 .0 as usize >= height || light_beams[j].0 .1 as usize >= length {
                if light_history
                    .get(&(light_beams[j].0, light_beams[j].1))
                    .is_some()
                {
                    light_beams.swap_remove(j);
                    continue;
                }
                light_beams.swap_remove(j);
                continue;
            }

            light_positions.insert(light_beams[j].0, true);

            let symbol = lines[light_beams[j].0 .0 as usize][light_beams[j].0 .1 as usize];
            if symbol == '.' {
                light_history.insert((light_beams[j].0, light_beams[j].1), true);
                continue;
            }
            let next_step = rules.get(&(symbol, light_beams[j].1)).expect("always have");
            if next_step.1.is_some() {
                light_beams[j].1 = next_step.0;
                if light_history
                    .get(&(light_beams[j].0, next_step.1.expect("checked for Some")))
                    .is_none()
                {
                    light_beams.push((light_beams[j].0, next_step.1.expect("checked for Some")));
                }

                if light_history
                    .get(&(light_beams[j].0, light_beams[j].1))
                    .is_some()
                {
                    light_beams.swap_remove(j);
                    continue;
                }
                light_history.insert(
                    (light_beams[j].0, next_step.1.expect("checked for Some")),
                    true,
                );
            } else {
                light_beams[j].1 = next_step.0;
                if light_history
                    .get(&(light_beams[j].0, light_beams[j].1))
                    .is_some()
                {
                    light_beams.swap_remove(j);
                    continue;
                }
                light_history.insert((light_beams[j].0, light_beams[j].1), true);
            }
        }
    }

    result = light_positions.keys().len();

    result
}

fn part_2(lines: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    let mut rules: HashMap<(char, Direction), (Direction, Option<Direction>)> = HashMap::new();
    rules.insert(('|', Direction::UP), (Direction::UP, None));
    rules.insert(('|', Direction::DOWN), (Direction::DOWN, None));
    rules.insert(
        ('|', Direction::LEFT),
        (Direction::UP, Some(Direction::DOWN)),
    );
    rules.insert(
        ('|', Direction::RIGHT),
        (Direction::UP, Some(Direction::DOWN)),
    );
    rules.insert(('-', Direction::RIGHT), (Direction::RIGHT, None));
    rules.insert(('-', Direction::LEFT), (Direction::LEFT, None));
    rules.insert(
        ('-', Direction::UP),
        (Direction::RIGHT, Some(Direction::LEFT)),
    );
    rules.insert(
        ('-', Direction::DOWN),
        (Direction::LEFT, Some(Direction::RIGHT)),
    );
    rules.insert(('/', Direction::UP), (Direction::RIGHT, None));
    rules.insert(('/', Direction::DOWN), (Direction::LEFT, None));
    rules.insert(('/', Direction::RIGHT), (Direction::UP, None));
    rules.insert(('/', Direction::LEFT), (Direction::DOWN, None));

    rules.insert(('\\', Direction::UP), (Direction::LEFT, None));
    rules.insert(('\\', Direction::DOWN), (Direction::RIGHT, None));
    rules.insert(('\\', Direction::RIGHT), (Direction::DOWN, None));
    rules.insert(('\\', Direction::LEFT), (Direction::UP, None));

    let height = lines.len();
    let length = lines[0].len();

    let mut temp_result = 0;
    for i in 0..(lines.len() * 2) {
        let mut light_positions: HashMap<(isize, isize), bool> = HashMap::new();

        let mut light_history: HashMap<((isize, isize), Direction), bool> = HashMap::new();

        let mut light_beams: Vec<((isize, isize), Direction)> = Vec::new();
        let initial_position;
        let initial_direction;
        if i >= lines.len() {
            initial_position = ((i - lines.len()) as isize, (lines[0].len() - 1) as isize);
            initial_direction = Direction::LEFT;
        } else {
            initial_position = (i as isize, 0 as isize);
            initial_direction = Direction::RIGHT;
        }

        light_beams.push((initial_position, initial_direction));
        light_positions.insert(initial_position, true);
        light_history.insert((initial_position, initial_direction), true);
        let initial_symbol = lines[initial_position.0 as usize][initial_position.1 as usize];
        if initial_symbol != '.' {
            let next_step = rules
                .get(&(initial_symbol, initial_direction))
                .expect("always have");
            if next_step.1.is_some() {
                light_beams[0].1 = next_step.0;
                light_beams.push((light_beams[0].0, next_step.1.expect("checked for Some")));
            } else {
                light_beams[0].1 = next_step.0;
            }
        }

        while light_beams.len() != 0 {
            for j in 0..light_beams.len() {
                if j >= light_beams.len() {
                    continue;
                }

                light_beams[j].0 .0 += light_beams[j].1.direction().0;
                light_beams[j].0 .1 += light_beams[j].1.direction().1;

                if light_beams[j].0 .0 < 0 || light_beams[j].0 .1 < 0 {
                    light_beams.swap_remove(j);
                    continue;
                }

                if light_beams[j].0 .0 as usize >= height || light_beams[j].0 .1 as usize >= length
                {
                    if light_history
                        .get(&(light_beams[j].0, light_beams[j].1))
                        .is_some()
                    {
                        light_beams.swap_remove(j);
                        continue;
                    }
                    light_beams.swap_remove(j);
                    continue;
                }

                light_positions.insert(light_beams[j].0, true);

                let symbol = lines[light_beams[j].0 .0 as usize][light_beams[j].0 .1 as usize];
                if symbol == '.' {
                    light_history.insert((light_beams[j].0, light_beams[j].1), true);
                    continue;
                }
                let next_step = rules.get(&(symbol, light_beams[j].1)).expect("always have");
                if next_step.1.is_some() {
                    light_beams[j].1 = next_step.0;
                    if light_history
                        .get(&(light_beams[j].0, next_step.1.expect("checked for Some")))
                        .is_none()
                    {
                        light_beams
                            .push((light_beams[j].0, next_step.1.expect("checked for Some")));
                    }

                    if light_history
                        .get(&(light_beams[j].0, light_beams[j].1))
                        .is_some()
                    {
                        light_beams.swap_remove(j);
                        continue;
                    }
                    light_history.insert(
                        (light_beams[j].0, next_step.1.expect("checked for Some")),
                        true,
                    );
                } else {
                    light_beams[j].1 = next_step.0;
                    if light_history
                        .get(&(light_beams[j].0, light_beams[j].1))
                        .is_some()
                    {
                        light_beams.swap_remove(j);
                        continue;
                    }
                    light_history.insert((light_beams[j].0, light_beams[j].1), true);
                }
            }
        }

        temp_result = light_positions.keys().len();

        if temp_result > result {
            result = temp_result;
        }
    }

    for i in 0..(lines[0].len() * 2) {
        let mut light_positions: HashMap<(isize, isize), bool> = HashMap::new();

        let mut light_history: HashMap<((isize, isize), Direction), bool> = HashMap::new();

        let mut light_beams: Vec<((isize, isize), Direction)> = Vec::new();
        let initial_position;
        let initial_direction;
        if i >= lines[0].len() {
            initial_position = ((lines.len() - 1) as isize, (i - lines[0].len()) as isize);
            initial_direction = Direction::UP;
        } else {
            initial_position = (0 as isize, i as isize);
            initial_direction = Direction::DOWN;
        }

        light_beams.push((initial_position, initial_direction));
        light_positions.insert(initial_position, true);
        light_history.insert((initial_position, initial_direction), true);
        let initial_symbol = lines[initial_position.0 as usize][initial_position.1 as usize];
        if initial_symbol != '.' {
            let next_step = rules
                .get(&(initial_symbol, initial_direction))
                .expect("always have");
            if next_step.1.is_some() {
                light_beams[0].1 = next_step.0;
                light_beams.push((light_beams[0].0, next_step.1.expect("checked for Some")));
            } else {
                light_beams[0].1 = next_step.0;
            }
        }

        while light_beams.len() != 0 {
            for j in 0..light_beams.len() {
                if j >= light_beams.len() {
                    continue;
                }

                light_beams[j].0 .0 += light_beams[j].1.direction().0;
                light_beams[j].0 .1 += light_beams[j].1.direction().1;

                if light_beams[j].0 .0 < 0 || light_beams[j].0 .1 < 0 {
                    light_beams.swap_remove(j);
                    continue;
                }

                if light_beams[j].0 .0 as usize >= height || light_beams[j].0 .1 as usize >= length
                {
                    if light_history
                        .get(&(light_beams[j].0, light_beams[j].1))
                        .is_some()
                    {
                        light_beams.swap_remove(j);
                        continue;
                    }
                    light_beams.swap_remove(j);
                    continue;
                }

                light_positions.insert(light_beams[j].0, true);

                let symbol = lines[light_beams[j].0 .0 as usize][light_beams[j].0 .1 as usize];
                if symbol == '.' {
                    light_history.insert((light_beams[j].0, light_beams[j].1), true);
                    continue;
                }
                let next_step = rules.get(&(symbol, light_beams[j].1)).expect("always have");
                if next_step.1.is_some() {
                    light_beams[j].1 = next_step.0;
                    if light_history
                        .get(&(light_beams[j].0, next_step.1.expect("checked for Some")))
                        .is_none()
                    {
                        light_beams
                            .push((light_beams[j].0, next_step.1.expect("checked for Some")));
                    }

                    if light_history
                        .get(&(light_beams[j].0, light_beams[j].1))
                        .is_some()
                    {
                        light_beams.swap_remove(j);
                        continue;
                    }
                    light_history.insert(
                        (light_beams[j].0, next_step.1.expect("checked for Some")),
                        true,
                    );
                } else {
                    light_beams[j].1 = next_step.0;
                    if light_history
                        .get(&(light_beams[j].0, light_beams[j].1))
                        .is_some()
                    {
                        light_beams.swap_remove(j);
                        continue;
                    }
                    light_history.insert((light_beams[j].0, light_beams[j].1), true);
                }
            }
        }

        temp_result = light_positions.keys().len();

        if temp_result > result {
            result = temp_result;
        }
    }

    result
}
