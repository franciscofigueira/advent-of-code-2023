use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents
        .split("\r\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    //println!("{lines:?}");
    let part_1_res = part_1(&mut lines.clone());
    println!("part 1 result {part_1_res:?}");

    let part_2_res = part_2(&mut lines.clone(), 1000000000);
    println!("part 2 result {part_2_res:?}");
}

fn part_1(lines: &mut Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for i in 1..lines.len() {
        for j in 0..lines[i].len() {
            let mut k = i;
            while k != 0 {
                if lines[k][j] == 'O' {
                    if lines[k - 1][j] == '.' {
                        lines[k - 1][j] = 'O';
                        lines[k][j] = '.';
                    } else {
                        break;
                    }
                }
                k -= 1;
            }
        }
    }

    //println!("{lines:?}");

    for i in 0..lines.len() {
        let number_of_rounded = lines[i].iter().filter(|&n| *n == 'O').count();
        result += number_of_rounded * (lines.len() - i);
    }

    result
}

fn part_2(lines: &mut Vec<Vec<char>>, num_cycles: usize) -> usize {
    let mut result = 0;

    let mut lines_copy: Vec<Vec<char>> = Vec::new();
    let mut cycle_duration = 0;
    for p in 0..num_cycles {
        if p == 1000 {
            lines_copy = lines.clone();
        }

        for i in 1..lines.len() {
            for j in 0..lines[i].len() {
                let mut k = i;
                while k != 0 {
                    if lines[k][j] == 'O' {
                        if lines[k - 1][j] == '.' {
                            lines[k - 1][j] = 'O';
                            lines[k][j] = '.';
                        } else {
                            break;
                        }
                    }
                    k -= 1;
                }
            }
        }

        for i in 0..lines.len() {
            for j in 1..lines[i].len() {
                let mut k = j;
                while k != 0 {
                    if lines[i][k] == 'O' {
                        if lines[i][k - 1] == '.' {
                            lines[i][k - 1] = 'O';
                            lines[i][k] = '.';
                        } else {
                            break;
                        }
                    }
                    k -= 1;
                }
            }
        }

        for i in 1..lines.len() {
            for j in 0..lines[i].len() {
                let mut k = lines.len() - i;
                while k != lines.len() {
                    if lines[k - 1][j] == 'O' {
                        if lines[k][j] == '.' {
                            lines[k][j] = 'O';
                            lines[k - 1][j] = '.';
                        } else {
                            break;
                        }
                    }
                    k += 1;
                }
            }
        }

        for i in 0..lines.len() {
            for j in 1..=lines[i].len() {
                let mut k = lines[i].len() - j;
                while k != lines[i].len() - 1 {
                    if lines[i][k] == 'O' {
                        if lines[i][k + 1] == '.' {
                            lines[i][k + 1] = 'O';
                            lines[i][k] = '.';
                        } else {
                            break;
                        }
                    }
                    k += 1;
                }
            }
        }

        if lines == &mut lines_copy && p != 1000 {
            cycle_duration = p - 1000;
            break;
        }
    }

    let number_times_to_cycle = (num_cycles - 1000) % cycle_duration;

    for _ in 0..=cycle_duration - number_times_to_cycle {
        for i in 1..lines.len() {
            for j in 0..lines[i].len() {
                let mut k = i;
                while k != 0 {
                    if lines[k][j] == 'O' {
                        if lines[k - 1][j] == '.' {
                            lines[k - 1][j] = 'O';
                            lines[k][j] = '.';
                        } else {
                            break;
                        }
                    }
                    k -= 1;
                }
            }
        }

        for i in 0..lines.len() {
            for j in 1..lines[i].len() {
                let mut k = j;
                while k != 0 {
                    if lines[i][k] == 'O' {
                        if lines[i][k - 1] == '.' {
                            lines[i][k - 1] = 'O';
                            lines[i][k] = '.';
                        } else {
                            break;
                        }
                    }
                    k -= 1;
                }
            }
        }

        for i in 1..lines.len() {
            for j in 0..lines[i].len() {
                let mut k = lines.len() - i;
                while k != lines.len() {
                    if lines[k - 1][j] == 'O' {
                        if lines[k][j] == '.' {
                            lines[k][j] = 'O';
                            lines[k - 1][j] = '.';
                        } else {
                            break;
                        }
                    }
                    k += 1;
                }
            }
        }

        for i in 0..lines.len() {
            for j in 1..=lines[i].len() {
                let mut k = lines[i].len() - j;
                while k != lines[i].len() - 1 {
                    if lines[i][k] == 'O' {
                        if lines[i][k + 1] == '.' {
                            lines[i][k + 1] = 'O';
                            lines[i][k] = '.';
                        } else {
                            break;
                        }
                    }
                    k += 1;
                }
            }
        }
    }

    for i in 0..lines.len() {
        let number_of_rounded = lines[i].iter().filter(|&n| *n == 'O').count();
        result += number_of_rounded * (lines.len() - i);
    }

    result
}
