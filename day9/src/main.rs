use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<Vec<i64>> = contents
        .split("\r\n")
        .map(|line| {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().expect("always number"))
                .collect();
            return numbers;
        })
        .collect();

    let part_1_res = part_1(&lines);
    println!("part 1 res: {part_1_res:?}");
    let part_2_res = part_2(&lines);
    println!("part 2 res: {part_2_res:?}");
}

fn part_1(lines: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for line in lines {
        let mut sequences = Vec::new();
        sequences.push(line.clone());

        while sequences[sequences.len() - 1]
            .iter()
            .filter(|&n| *n == 0)
            .count()
            != sequences[sequences.len() - 1].len()
        {
            let mut new_sequence = Vec::new();
            for i in 0..sequences[sequences.len() - 1].len() - 1 {
                new_sequence.push(
                    sequences[sequences.len() - 1][i + 1] - sequences[sequences.len() - 1][i],
                );
            }
            sequences.push(new_sequence);
        }

        let mut number = 0;
        for sequence in sequences {
            number += sequence[sequence.len() - 1];
        }
        result += number;
    }

    result
}

fn part_2(lines: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for line in lines {
        let mut sequences = Vec::new();
        sequences.push(line.clone());

        while sequences[sequences.len() - 1]
            .iter()
            .filter(|&n| *n == 0)
            .count()
            != sequences[sequences.len() - 1].len()
        {
            let mut new_sequence = Vec::new();
            for i in 0..sequences[sequences.len() - 1].len() - 1 {
                new_sequence.push(
                    sequences[sequences.len() - 1][i + 1] - sequences[sequences.len() - 1][i],
                );
            }
            sequences.push(new_sequence);
        }

        let mut number = 0;
        for i in 0..sequences.len() {
            let sequence = &sequences[i];
            if i % 2 == 0 {
                number += sequence[0];
            } else {
                number -= sequence[0];
            }
        }
        result += number;
    }

    result
}
