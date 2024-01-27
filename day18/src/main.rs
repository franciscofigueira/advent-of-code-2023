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
    let lines: Vec<_> = contents
        .split("\r\n")
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let part_1_res = part_1(&lines);
    println!("part 1 result {part_1_res:?}");
    let part_2_res = part_2(&lines);
    println!("part 2 result {part_2_res:?}");
}

fn part_1(lines: &Vec<Vec<&str>>) -> usize {
    let mut dig_points: Vec<(isize, isize)> = Vec::new();
    let mut current_position = (0, 0);

    let mut perimeter = 0;
    dig_points.push(current_position);
    for line in lines {
        let direction = line[0];
        let distance = line[1].parse::<isize>().expect("always number");

        if direction == "U" {
            current_position.0 += Direction::UP.direction().0 * distance;
            current_position.1 += Direction::UP.direction().1 * distance;
        } else if direction == "D" {
            current_position.0 += Direction::DOWN.direction().0 * distance;
            current_position.1 += Direction::DOWN.direction().1 * distance;
        } else if direction == "R" {
            current_position.0 += Direction::RIGHT.direction().0 * distance;
            current_position.1 += Direction::RIGHT.direction().1 * distance;
        } else {
            current_position.0 += Direction::LEFT.direction().0 * distance;
            current_position.1 += Direction::LEFT.direction().1 * distance;
        }
        perimeter += distance;
        dig_points.push(current_position);
    }

    let mut area = 0;
    for i in 0..dig_points.len() - 1 {
        area += (dig_points[i].0 + dig_points[i + 1].0) * (dig_points[i].1 - dig_points[i + 1].1);
    }
    area += (dig_points[dig_points.len() - 1].0 + dig_points[0].0)
        * (dig_points[dig_points.len() - 1].1 - dig_points[0].1);

    area = area.abs() / 2;

    let result = area as usize + perimeter as usize / 2 + 1;

    result
}

fn part_2(lines: &Vec<Vec<&str>>) -> usize {
    let mut current_position = (0, 0);
    let mut dig_points: Vec<(isize, isize)> = Vec::new();

    let mut perimeter = 0;

    for line in lines {
        let (distance, direction) = line[2].split_at(line[2].len() - 2);
        let (direction, _) = direction.split_at(1);
        let (_, distance) = distance.split_at(2);
        let distance = isize::from_str_radix(distance, 16).expect("always number");

        if direction == "0" {
            current_position.0 += Direction::RIGHT.direction().0 * distance;
            current_position.1 += Direction::RIGHT.direction().1 * distance;
        } else if direction == "1" {
            current_position.0 += Direction::DOWN.direction().0 * distance;
            current_position.1 += Direction::DOWN.direction().1 * distance;
        } else if direction == "2" {
            current_position.0 += Direction::LEFT.direction().0 * distance;
            current_position.1 += Direction::LEFT.direction().1 * distance;
        } else {
            current_position.0 += Direction::UP.direction().0 * distance;
            current_position.1 += Direction::UP.direction().1 * distance;
        }
        perimeter += distance;
        dig_points.push(current_position);
    }

    let mut area = 0;
    for i in 0..dig_points.len() - 1 {
        area += (dig_points[i].0 + dig_points[i + 1].0) * (dig_points[i].1 - dig_points[i + 1].1);
    }
    area += (dig_points[dig_points.len() - 1].0 + dig_points[0].0)
        * (dig_points[dig_points.len() - 1].1 - dig_points[0].1);

    area = area.abs() / 2;

    let result = area as usize + perimeter as usize / 2 + 1;

    result
}
