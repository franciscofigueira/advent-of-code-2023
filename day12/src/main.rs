use std::collections::HashMap;
use std::io::Read;
use std::thread;

#[derive(Debug, Clone)]
struct Spring {
    conditions: Vec<char>,
    groups: Vec<usize>,
}

fn main() {
    let mut file = std::fs::File::open("./test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines: Vec<Spring> = contents
        .lines()
        .map(|x| {
            let (springs, groups) = x.split_once(" ").expect("awlways have space");
            return Spring {
                conditions: springs.chars().collect(),
                groups: groups
                    .split(",")
                    .map(|x| x.parse::<usize>().expect("always number"))
                    .collect(),
            };
        })
        .collect();

    let part_1_res = part_1(&lines);
    println!("part 1 res: {part_1_res:?}");
    let part_2_res = part_2(&mut lines);
    println!("part 2 res: {part_2_res:?}");
}

fn part_1(lines: &Vec<Spring>) -> usize {
    let mut result = 0;

    let threads: Vec<_> = (0..lines.len())
        .map(|i| {
            let line = lines[i].clone();
            thread::spawn(move || {
                let mut line_result = 0;
                let total_number_damaged_parts: usize = line.groups.iter().sum();
                let number_known_damaged_parts: usize =
                    line.conditions.iter().filter(|&n| *n == '#').count();
                let number_unknown_parts: usize =
                    line.conditions.iter().filter(|&n| *n == '?').count();

                let mut vec_poss: Vec<char> =
                    vec!['#'; total_number_damaged_parts - number_known_damaged_parts];
                let mut vec2: Vec<char> = vec![
                    '.';
                    number_unknown_parts
                        - (total_number_damaged_parts
                            - number_known_damaged_parts)
                ];
                vec_poss.append(&mut vec2);

                let mut result: Vec<Vec<char>> = Vec::new();

                permute(0, vec_poss, &mut result);
                for perm in result {
                    let mut j = 0;
                    let mut cl = line.conditions.clone();
                    for i in 0..cl.len() {
                        if cl[i] == '?' {
                            cl[i] = perm[j];
                            j += 1;
                        }
                    }

                    let res = is_valid(&cl, &line.groups);
                    if res {
                        line_result += 1;
                    }
                }

                return line_result;
            })
        })
        .collect();
    for handle in threads {
        let res = handle.join().unwrap();
        result += res;
    }

    result
}

fn part_2(lines: &mut Vec<Spring>) -> usize {
    let mut result = 0;

    for i in 0..lines.len() {
        lines[i].conditions.push('?');
        let append_1 = lines[i].conditions.clone();
        let append_2 = lines[i].groups.clone();

        for _ in 0..5 {
            lines[i].conditions.append(&mut append_1.clone());
            lines[i].groups.append(&mut append_2.clone());
        }
        lines[i].conditions.pop();
    }
    //println!("{lines:?}");

    let mut cache: HashMap<(usize, usize, usize), u64> = HashMap::new();

    for line in lines {}

    result as usize
}

fn is_valid(attempt: &Vec<char>, groups: &Vec<usize>) -> bool {
    let res = true;

    let mut count = 0;
    let mut iter = attempt.iter().peekable();
    while let Some(ch) = iter.peek() {
        if **ch != '#' {
            iter.next();
        } else {
            break;
        }
    }
    'outer: for group in groups {
        while let Some(ch) = iter.next() {
            if *ch == '#' {
                count += 1;
            } else {
                if group != &count {
                    return false;
                } else {
                    count = 0;
                    while let Some(ch) = iter.peek() {
                        if **ch != '#' {
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    continue 'outer;
                }
            }
        }
        if group != &count {
            return false;
        }
    }

    res
}

fn permute(i: usize, s: Vec<char>, result: &mut Vec<Vec<char>>) {
    if i == (s.len() - 1) {
        result.push(s);

        return;
    }
    let mut prev = '*';
    for j in i..s.len() {
        let mut temp: Vec<char> = s.clone();
        if j > i && temp[i] == temp[j] {
            continue;
        }

        if prev != '*' && prev == s[j] {
            continue;
        }

        temp.swap(i, j);
        prev = s[j];

        permute(i + 1, temp, result);
    }
}

#[test]
fn tes() {
    let initial = vec!['#', '#', '.'];

    let mut result: Vec<Vec<char>> = Vec::new();
    permute(0, initial, &mut result);

    println!(" sasd{result:?}");
}
