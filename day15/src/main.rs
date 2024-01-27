use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split(",").collect();
    // println!("{lines:?}");
    let part_1_res = part1(&lines);
    println!("part 1 result: {part_1_res}");
    let part_2_res = part2(&lines);
    println!("part 2 result: {part_2_res}");
}

fn part1(lines: &Vec<&str>) -> usize {
    let mut result = 0;
    for line in lines {
        result += hash(line.chars().collect());
    }

    result
}

fn part2(lines: &Vec<&str>) -> usize {
    let mut result = 0;

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::with_capacity(1); 256];

    for line in lines {
        if line.contains("=") {
            let (label, focal_length) = line.split_once("=").expect("always have =");
            let box_number = hash(label.chars().collect());
            let mut found_label = false;
            for i in 0..boxes[box_number].len() {
                let box_label = boxes[box_number][i].0;
                if label == box_label {
                    found_label = true;
                    boxes[box_number][i].1 = focal_length.parse::<usize>().expect("always number");
                    break;
                }
            }
            if found_label == false {
                boxes[box_number].insert(
                    0,
                    (label, focal_length.parse::<usize>().expect("always number")),
                );
            }
        } else {
            let (label, _) = line.split_once("-").expect("always have -");
            let box_number = hash(label.chars().collect());
            for i in 0..boxes[box_number].len() {
                let box_label = boxes[box_number][i].0;
                if label == box_label {
                    boxes[box_number].remove(i);
                    break;
                }
            }
        }
    }
    // println!("part_2: {boxes:?}");

    for j in 0..boxes.len() {
        let lens_box = &boxes[j];
        for i in 0..lens_box.len() {
            let box1 = lens_box[i];
            let len = lens_box.len();
            println!("{box1:?}, {i}, {len}");
            result += lens_box[i].1 * (lens_box.len() - i) * (j + 1);
        }
    }

    result
}

fn hash(input: Vec<char>) -> usize {
    let mut digest = 0;

    for char in input {
        let ascii_code = char.to_string().as_bytes()[0];
        digest += ascii_code as usize;
        digest *= 17;
        digest = digest % 256;
    }

    digest
}

#[test]
fn test_hash() {
    let input = vec!['H', 'A', 'S', 'H'];
    let res = hash(input);

    assert_eq!(res, 52);
}
