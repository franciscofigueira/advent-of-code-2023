use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r\n").collect();

    let part_1_res = part_1(&lines);
    println!("part 1 result {part_1_res}");

    let part_2_res = part_2(&lines);
    println!("part 2 result {part_2_res}");
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut result = 0;

    for line in lines {
        let mut card_points = 0;
        let (_, winning_possessed_numbers) = line.split_once(": ").expect("always have card n:");
        let (winning_numbers, possessed_numbers) = winning_possessed_numbers
            .split_once("|")
            .expect("numbers always seperated by |");

        let vec_winning_numbers: Vec<_> = winning_numbers.split_whitespace().collect();
        let vec_possessed_numbers: Vec<_> = possessed_numbers.split_whitespace().collect();

        for winning_number in vec_winning_numbers {
            if vec_possessed_numbers.contains(&winning_number) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        result += card_points;
    }

    result
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut result = 0;

    let mut number_of_cards_per_id = vec![1; lines.len()];

    let mut i = 0;
    for line in lines {
        let mut card_points = 0;
        let (_, winning_possessed_numbers) = line.split_once(": ").expect("always have card n:");
        let (winning_numbers, possessed_numbers) = winning_possessed_numbers
            .split_once("|")
            .expect("numbers always seperated by |");

        let vec_winning_numbers: Vec<_> = winning_numbers.split_whitespace().collect();
        let vec_possessed_numbers: Vec<_> = possessed_numbers.split_whitespace().collect();
        for winning_number in vec_winning_numbers {
            if vec_possessed_numbers.contains(&winning_number) {
                card_points += 1;
            }
        }

        for n in 0..card_points {
            number_of_cards_per_id[i + n + 1] += number_of_cards_per_id[i];
        }

        i += 1;
    }

    result = number_of_cards_per_id.iter().sum();

    result
}
