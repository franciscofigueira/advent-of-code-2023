use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r\n").collect();

    let part_1_res = part_1(&lines);
    let part_2_res = part_2(&lines);
    println!("part 1 result: {part_1_res}");
    println!("part 2 result: {part_2_res}");
}

#[derive(Debug, Clone)]
struct Num {
    value: String,
    start_index: usize,
    end_index: usize,
}
impl Num {
    pub fn default() -> Self {
        Num {
            value: String::new(),
            start_index: 0,
            end_index: 0,
        }
    }
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut total = 0;

    let mut numbers_in_line: Vec<Vec<Num>> = Vec::new();
    let mut symbols_in_line: Vec<Vec<usize>> = Vec::new();
    let mut line_counter = 0;

    for line in lines {
        numbers_in_line.push(Vec::new());
        symbols_in_line.push(Vec::new());

        let mut i = 0;
        let mut in_number = false;
        let mut current_number: Num = Num::default();
        for char in line.chars() {
            if char != '.' {
                if char.is_digit(10) {
                    if !in_number {
                        in_number = true;
                        current_number.value += char.to_string().as_str();
                        current_number.start_index = i;
                        current_number.end_index = i;
                    } else {
                        current_number.value += char.to_string().as_str();
                        current_number.end_index = i;
                    }
                } else {
                    if in_number {
                        in_number = false;
                        numbers_in_line[line_counter].push(current_number);
                        symbols_in_line[line_counter].push(i);
                        current_number = Num::default();
                    } else {
                        symbols_in_line[line_counter].push(i);
                    }
                }
            } else {
                if in_number {
                    in_number = false;
                    numbers_in_line[line_counter].push(current_number);
                    current_number = Num::default();
                }
            }

            i += 1;
        }
        if current_number.value != String::new() {
            numbers_in_line[line_counter].push(current_number);
        }
        line_counter += 1;
    }

    let mut i = 0;
    for line in numbers_in_line {
        for num in line {
            if has_symbol_next_to_number(&num, i, &symbols_in_line) {
                total += num.value.parse::<u32>().unwrap();
            }
        }

        i += 1;
    }

    total
}

fn has_symbol_next_to_number(
    number: &Num,
    line_number: usize,
    symbols_in_line: &Vec<Vec<usize>>,
) -> bool {
    let total_num_lines = symbols_in_line.len();

    if line_number == 0 {
        for symbol in &symbols_in_line[0] {
            if *symbol == number.end_index + 1 || *symbol + 1 == number.start_index {
                return true;
            }
        }
        for symbol in &symbols_in_line[1] {
            if *symbol <= number.end_index + 1 && *symbol + 1 >= number.start_index {
                return true;
            }
        }
    } else if line_number == total_num_lines - 1 {
        for symbol in &symbols_in_line[line_number] {
            if *symbol == number.end_index + 1 || *symbol + 1 == number.start_index {
                return true;
            }
        }
        for symbol in &symbols_in_line[line_number - 1] {
            if *symbol <= number.end_index + 1 && *symbol + 1 >= number.start_index {
                return true;
            }
        }
    } else {
        for symbol in &symbols_in_line[line_number] {
            if *symbol == number.end_index + 1 || *symbol + 1 == number.start_index {
                return true;
            }
        }
        for symbol in &symbols_in_line[line_number - 1] {
            if *symbol <= number.end_index + 1 && *symbol + 1 >= number.start_index {
                return true;
            }
        }
        for symbol in &symbols_in_line[line_number + 1] {
            if *symbol <= number.end_index + 1 && *symbol + 1 >= number.start_index {
                return true;
            }
        }
    }

    return false;
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut total = 0;

    let mut numbers_in_line: Vec<Vec<Num>> = Vec::new();
    let mut asterisk_in_line: Vec<Vec<usize>> = Vec::new();
    let mut line_counter = 0;

    for line in lines {
        numbers_in_line.push(Vec::new());
        asterisk_in_line.push(Vec::new());

        let mut i = 0;
        let mut in_number = false;
        let mut current_number: Num = Num::default();
        for char in line.chars() {
            if char != '.' {
                if char.is_digit(10) {
                    if !in_number {
                        in_number = true;
                        current_number.value += char.to_string().as_str();
                        current_number.start_index = i;
                        current_number.end_index = i;
                    } else {
                        current_number.value += char.to_string().as_str();
                        current_number.end_index = i;
                    }
                } else {
                    if in_number {
                        in_number = false;
                        numbers_in_line[line_counter].push(current_number);
                        if char == '*' {
                            asterisk_in_line[line_counter].push(i);
                        }

                        current_number = Num::default();
                    } else {
                        if char == '*' {
                            asterisk_in_line[line_counter].push(i);
                        }
                    }
                }
            } else {
                if in_number {
                    in_number = false;
                    numbers_in_line[line_counter].push(current_number);
                    current_number = Num::default();
                }
            }

            i += 1;
        }
        if current_number.value != String::new() {
            numbers_in_line[line_counter].push(current_number);
        }
        line_counter += 1;
    }

    let mut i = 0;
    for line in asterisk_in_line {
        for asterisk in line {
            total += is_gear(i, asterisk, &numbers_in_line);
        }
        i += 1;
    }

    total
}

fn is_gear(line_number: usize, position_in_line: usize, numbers: &Vec<Vec<Num>>) -> u32 {
    let mut res = 0;
    let mut first_number = Num::default();
    let mut second_number = Num::default();
    let total_num_lines = numbers.len();
    let mut number_of_adjacent_numbers = 0;

    if line_number == 0 {
        for number in &numbers[0] {
            if number.end_index + 1 == position_in_line
                || number.start_index == position_in_line + 1
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
        for number in &numbers[1] {
            if position_in_line <= number.end_index + 1
                && position_in_line + 1 >= number.start_index
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
    } else if line_number == total_num_lines - 1 {
        for number in &numbers[line_number] {
            if number.end_index + 1 == position_in_line
                || number.start_index == position_in_line + 1
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
        for number in &numbers[line_number - 1] {
            if position_in_line <= number.end_index + 1
                && position_in_line + 1 >= number.start_index
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
    } else {
        for number in &numbers[line_number] {
            if number.end_index + 1 == position_in_line
                || number.start_index == position_in_line + 1
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
        for number in &numbers[line_number - 1] {
            if position_in_line <= number.end_index + 1
                && position_in_line + 1 >= number.start_index
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
        for number in &numbers[line_number + 1] {
            if position_in_line <= number.end_index + 1
                && position_in_line + 1 >= number.start_index
            {
                number_of_adjacent_numbers += 1;
                if first_number.value == String::new() {
                    first_number = number.clone();
                } else {
                    second_number = number.clone();
                }
            }
        }
    }
    // println!("f {first_number:?}");
    // println!("s {second_number:?}");
    // println!("n {number_of_adjacent_numbers}");
    if first_number.value != String::new()
        && second_number.value != String::new()
        && number_of_adjacent_numbers == 2
    {
        res = first_number.value.parse::<u32>().unwrap()
            * second_number.value.parse::<u32>().unwrap();
    }
    res
}
