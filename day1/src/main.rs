use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r").collect();

    let total_1 = part_1(&lines);
    let total_2 = part_2(&lines);

    println!("part 1: {:?}", total_1);
    println!("part 2: {:?}", total_2);
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in lines {
        let mut first: char = 'a';
        let mut last: char = 'b';
        for c in line.chars() {
            if c.is_digit(10) {
                if first == 'a' {
                    first = c;
                } else {
                    last = c;
                }
            }
        }

        if last == 'b' {
            last = first;
        }
        let num = first.to_string() + last.to_string().as_str();
        total += num.parse::<u32>().unwrap();
    }
    total
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total = 0;
    for line in lines {
        let mut i = 1;
        let mut current_digit_post: Vec<_>;

        let mut first_digit_value = usize::MAX;
        let mut first_digit_position = usize::MAX;

        let mut last_digit_value = usize::MAX;
        let mut last_digit_position = 0;

        for digit in digits {
            if line.contains(digit) {
                current_digit_post = line.match_indices(digit).collect();

                if current_digit_post.get(0).unwrap().0 < first_digit_position {
                    first_digit_value = i;
                    first_digit_position = current_digit_post.get(0).unwrap().0;
                }

                if current_digit_post
                    .get(current_digit_post.len() - 1)
                    .unwrap()
                    .0
                    >= last_digit_position
                {
                    last_digit_value = i;
                    last_digit_position = current_digit_post
                        .get(current_digit_post.len() - 1)
                        .unwrap()
                        .0;
                }
            }

            i += 1;
        }

        let mut first: char = 'a';
        let mut last: char = 'b';
        let mut i: usize = 0;
        let mut last_char_digit_pos = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                if first == 'a' {
                    if i < first_digit_position {
                        first = c;
                    } else {
                        first = std::char::from_digit(first_digit_value as u32, 10).unwrap();
                        last = c;
                        last_char_digit_pos = i;
                    }
                } else {
                    last = c;
                    last_char_digit_pos = i;
                }
            }
            i += 1;
        }

        if first == 'a' {
            first = std::char::from_digit(first_digit_value as u32, 10).unwrap();
        }
        if last_digit_position > last_char_digit_pos {
            last = std::char::from_digit(last_digit_value as u32, 10).unwrap();
        }
        if last == 'b' {
            last = first;
        }
        let num = first.to_string() + last.to_string().as_str();

        total += num.parse::<u32>().unwrap();
    }

    total
}
