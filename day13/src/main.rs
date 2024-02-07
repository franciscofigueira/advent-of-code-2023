use std::{collections::VecDeque, io::Read};

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<Vec<Vec<char>>> = contents
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| y.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut history: VecDeque<usize> = VecDeque::new();

    let part_1_res = part_1(&lines, &mut history);
    println!("part 1 result {part_1_res:?}");
    let part_2_res = part_2(&lines, &mut history);
    println!("part 2 result {part_2_res:?}");
}

fn part_1(lines: &Vec<Vec<Vec<char>>>, history: &mut VecDeque<usize>) -> usize {
    let mut result = 0;

    'outer: for pattern in lines {
        //check horizontal

        for j in 1..(pattern.len() - 1) {
            for i in 0..(pattern.len() / 2 + 1) {
                if pattern[j - i] != pattern[j + 1 + i] {
                    break;
                }
                if j - i == 0 || j + 1 + i == pattern.len() - 1 {
                    history.push_back((j + 1) * 100);
                    result += (j + 1) * 100;
                    continue 'outer;
                }
            }
            for i in 0..(pattern.len() / 2) {
                if pattern[j - i - 1] != pattern[j + i] {
                    break;
                }
                if j - i - 1 == 0 || j + i == pattern.len() - 1 {
                    history.push_back(j * 100);
                    result += j * 100;
                    continue 'outer;
                }
            }
        }

        //check vertical
        let n_columns = pattern[0].len();
        for j in 1..(n_columns - 1) {
            for i in 0..(n_columns / 2 + 1) {
                let col_1 = get_vec_of_column(pattern, j - i);
                let col_2 = get_vec_of_column(pattern, j + 1 + i);
                if col_1 != col_2 {
                    break;
                }
                if j - i == 0 || j + 1 + i == n_columns - 1 {
                    history.push_back(j + 1);
                    result += j + 1;
                    continue 'outer;
                }
            }
            for i in 0..(n_columns / 2) {
                let col_1 = get_vec_of_column(pattern, j - i - 1);
                let col_2 = get_vec_of_column(pattern, j + i);
                if col_1 != col_2 {
                    break;
                }
                if j - i - 1 == 0 || j + i == n_columns - 1 {
                    history.push_back(j);
                    result += j;
                    continue 'outer;
                }
            }
        }
    }

    result
}

fn part_2(lines: &Vec<Vec<Vec<char>>>, history: &mut VecDeque<usize>) -> usize {
    let mut result = 0;

    'outer: for pattern in lines {
        let old_reflection_value = history
            .pop_front()
            .expect("will have same number of patterns");
        //check horizontal

        for row in 0..pattern.len() {
            for col in 0..pattern[0].len() {
                let mut pattern_clone = pattern.clone();
                if pattern_clone[row][col] == '#' {
                    pattern_clone[row][col] = '.'
                } else {
                    pattern_clone[row][col] = '#'
                }

                for j in 1..(pattern_clone.len() - 1) {
                    for i in 0..(pattern_clone.len() / 2 + 1) {
                        if j + 1 + i <= pattern_clone.len() - 1
                            && j >= i
                            && pattern_clone[j - i] != pattern_clone[j + 1 + i]
                        {
                            break;
                        }

                        if (j - i == 0 || j + 1 + i == pattern_clone.len() - 1)
                            && old_reflection_value != (j + 1) * 100
                        {
                            result += (j + 1) * 100;
                            continue 'outer;
                        }
                    }
                    for i in 0..(pattern_clone.len() / 2) {
                        if j + i <= pattern_clone.len() - 1
                            && j >= i + 1
                            && pattern_clone[j - i - 1] != pattern_clone[j + i]
                        {
                            break;
                        }
                        if (j - i - 1 == 0 || j + i == pattern_clone.len() - 1)
                            && old_reflection_value != j * 100
                        {
                            result += j * 100;
                            continue 'outer;
                        }
                    }
                }

                //check vertical
                let n_columns = pattern_clone[0].len();
                for j in 1..(n_columns - 1) {
                    for i in 0..(n_columns / 2 + 1) {
                        if j + 1 + i <= pattern_clone[0].len() - 1 && j >= i {
                            let col_1 = get_vec_of_column(&pattern_clone, j - i);
                            let col_2 = get_vec_of_column(&pattern_clone, j + 1 + i);
                            if col_1 != col_2 {
                                break;
                            }
                            if (j - i == 0 || j + 1 + i == n_columns - 1)
                                && old_reflection_value != j + 1
                            {
                                result += j + 1;
                                continue 'outer;
                            }
                        }
                    }
                    for i in 0..(n_columns / 2) {
                        if j + i <= pattern_clone[0].len() - 1 && j >= i + 1 {
                            let col_1 = get_vec_of_column(&pattern_clone, j - i - 1);
                            let col_2 = get_vec_of_column(&pattern_clone, j + i);
                            if col_1 != col_2 {
                                break;
                            }
                            if (j - i - 1 == 0 || j + i == n_columns - 1)
                                && old_reflection_value != j
                            {
                                result += j;
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

fn get_vec_of_column(pattern: &Vec<Vec<char>>, column_index: usize) -> Vec<char> {
    let collum: Vec<char> = pattern
        .iter() // iterate over rows
        .map(|x| x[column_index]) // get the icolumn-th element from each row
        .collect(); //
    return collum;
}
