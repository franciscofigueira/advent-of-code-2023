use std::collections::{HashMap, HashSet};
use std::io::Read;

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

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<Vec<char>> = contents
        .split("\r\n")
        .map(|x| x.chars().collect())
        .collect();

    let mut rules: HashMap<(char, Direction), Direction> = HashMap::new();
    rules.insert(('|', Direction::UP), Direction::UP);
    rules.insert(('|', Direction::DOWN), Direction::DOWN);
    rules.insert(('-', Direction::RIGHT), Direction::RIGHT);
    rules.insert(('-', Direction::LEFT), Direction::LEFT);
    rules.insert(('L', Direction::DOWN), Direction::RIGHT);
    rules.insert(('L', Direction::LEFT), Direction::UP);
    rules.insert(('J', Direction::DOWN), Direction::LEFT);
    rules.insert(('J', Direction::RIGHT), Direction::UP);
    rules.insert(('7', Direction::UP), Direction::LEFT);
    rules.insert(('7', Direction::RIGHT), Direction::DOWN);
    rules.insert(('F', Direction::UP), Direction::RIGHT);
    rules.insert(('F', Direction::LEFT), Direction::DOWN);

    //println!("{lines:?}");
    let part_1_res = part_1(&lines, &rules);
    println!("part 1 res: {part_1_res:?}");
    let part_2_res = part_2(&lines, &rules);
    println!("part 2 res: {part_2_res:?}");
}

fn part_1(lines: &Vec<Vec<char>>, rules: &HashMap<(char, Direction), Direction>) -> u64 {
    let mut result = 0;
    let mut s_position: (usize, usize) = (0, 0);

    'outer: for i in 0..lines.len() {
        if lines[i].contains(&'S') {
            for j in 0..lines[i].len() {
                if lines[i][j] == 'S' {
                    s_position = (i, j);
                    break 'outer;
                }
            }
        }
    }

    let mut current_direction: Direction = Direction::DOWN;

    if s_position.0 != 0 {
        let up = lines[s_position.0 - 1][s_position.1];
        if up == '|' || up == 'F' || up == '7' {
            current_direction = Direction::UP;
        }
    }
    let down = lines[s_position.0 + 1][s_position.1];
    if down == '|' || down == 'J' || down == 'L' {
        current_direction = Direction::DOWN;
    }
    let left = lines[s_position.0][s_position.1 - 1];
    if left == '-' || left == 'J' || left == '7' {
        current_direction = Direction::LEFT;
    }

    let mut current_pos: (isize, isize) = (0, 0);
    current_pos.0 = s_position.0 as isize;
    current_pos.1 = s_position.1 as isize;

    current_pos.0 += current_direction.direction().0;
    current_pos.1 += current_direction.direction().1;

    while lines[current_pos.0 as usize][current_pos.1 as usize] != 'S' {
        let current_symbol = lines[current_pos.0 as usize][current_pos.1 as usize];

        let direction_clone = current_direction.clone();
        current_direction = rules
            .get(&(current_symbol, direction_clone))
            .expect("always exist")
            .clone();
        current_pos.0 += current_direction.direction().0;
        current_pos.1 += current_direction.direction().1;
        result += 1;
    }

    result / 2 + 1
}

fn part_2(lines: &Vec<Vec<char>>, rules: &HashMap<(char, Direction), Direction>) -> u64 {
    let mut result = 0;

    let mut s_position: (usize, usize) = (0, 0);
    let mut loop_points: Vec<(isize, isize)> = Vec::new();

    'outer: for i in 0..lines.len() {
        if lines[i].contains(&'S') {
            for j in 0..lines[i].len() {
                if lines[i][j] == 'S' {
                    s_position = (i, j);
                    break 'outer;
                }
            }
        }
    }

    let mut current_direction: Direction = Direction::DOWN;

    let down = lines[s_position.0 + 1][s_position.1];
    if down == '|' || down == 'J' || down == 'L' {
        current_direction = Direction::DOWN;
    }
    if s_position.0 != 0 {
        let up = lines[s_position.0 - 1][s_position.1];
        if up == '|' || up == 'F' || up == '7' {
            current_direction = Direction::UP;
        }
    }
    let left = lines[s_position.0][s_position.1 - 1];
    if left == '-' || left == 'F' || left == 'L' {
        current_direction = Direction::LEFT;
    }
    let right = lines[s_position.0][s_position.1 + 1];
    if right == '-' || right == 'J' || right == '7' {
        current_direction = Direction::RIGHT;
    }

    let mut current_pos: (isize, isize) = (0, 0);
    current_pos.0 = s_position.0 as isize;
    current_pos.1 = s_position.1 as isize;

    loop_points.push(current_pos);
    current_pos.0 += current_direction.direction().0;
    current_pos.1 += current_direction.direction().1;
    loop_points.push(current_pos);

    while lines[current_pos.0 as usize][current_pos.1 as usize] != 'S' {
        let current_symbol = lines[current_pos.0 as usize][current_pos.1 as usize];

        let current_direction_clone = current_direction.clone();
        current_direction = rules
            .get(&(current_symbol, current_direction_clone))
            .expect("always exist")
            .clone();

        current_pos.0 += current_direction.direction().0;
        current_pos.1 += current_direction.direction().1;
        loop_points.push(current_pos);
    }
    loop_points.pop();

    let mut area = 0;
    for i in 0..loop_points.len() - 1 {
        area +=
            (loop_points[i].0 + loop_points[i + 1].0) * (loop_points[i].1 - loop_points[i + 1].1);
    }
    area += (loop_points[loop_points.len() - 1].0 + loop_points[0].0)
        * (loop_points[loop_points.len() - 1].1 - loop_points[0].1);

    area = area / 2;
    area = area.abs();
    result = (area - loop_points.len() as isize / 2 + 1) as u64;

    result
}
