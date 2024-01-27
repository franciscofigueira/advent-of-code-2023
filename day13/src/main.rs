use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<Vec<Vec<char>>> = contents
        .split("\r\n\r\n")
        .map(|x| {
            x.split("\r\n")
                .map(|y| y.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    // println!("{lines:?}");
    let part_1_res = part_1(&lines);
    println!("part 1 result {part_1_res:?}");
}

fn part_1(lines: &Vec<Vec<Vec<char>>>) -> usize {
    let mut result = 0;

    for line in lines {
        let half = line.len() / 2;
        if line[half] == line[half + 1] {
            let fg: Vec<char> = line[half].clone();
            // println!("here {half},{fg:?}");
            result += (line.len() - half) * 100;
        } else {
            let half = line[0].len() / 2;
            let collum1: Vec<_> = line
                .iter() // iterate over rows
                .map(|x| x[half]) // get the icolumn-th element from each row
                .collect(); //
            let collum2: Vec<_> = line
                .iter() // iterate over rows
                .map(|x| x[half + 1]) // get the icolumn-th element from each row
                .collect(); //
            if collum1 == collum2 {
                println!("here {half},{collum1:?}");
                result += line[0].len() - half;
            }
        }
    }

    result
}
