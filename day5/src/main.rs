use std::{io::Read, thread};

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines: Vec<_> = contents.split("\r\n").collect();
    lines.retain(|&x| x != "");

    let part_1_result = part_1(&mut lines);
    println!("{part_1_result:?}");
    let part_2_result = part_2(&mut lines);
    println!("{part_2_result:?}");
}

fn part_1(lines: &mut Vec<&str>) -> u64 {
    let mut result = u64::MAX;

    let seeds: Vec<u64> = lines[0]
        .split_once(":")
        .expect("always have :")
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("always number"))
        .collect();

    let mut alamac: Vec<Vec<Vec<u64>>> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    for line in lines {
        if i == 0 {
            i += 1;
            continue;
        }
        if line
            .chars()
            .next()
            .expect("wil always have char")
            .is_alphabetic()
        {
            alamac.push(Vec::new());
            j += 1;
        } else {
            alamac[j - 1].push(
                line.split_whitespace()
                    .map(|x| x.parse::<u64>().expect("always number"))
                    .collect(),
            );
        }

        i += 1;
    }

    for seed in seeds {
        let seed_number = seed;
        let mut current_mapping = seed_number;
        for maps in &alamac {
            for map in maps {
                let length = map[2];
                let destination_range_start = map[0];
                let source_range_start = map[1];

                if current_mapping < source_range_start + length
                    && current_mapping >= source_range_start
                {
                    current_mapping =
                        destination_range_start + (current_mapping - source_range_start);
                    break;
                }
            }
        }
        if current_mapping < result {
            result = current_mapping;
        }
    }

    result
}

fn part_2(lines: &mut Vec<&str>) -> u64 {
    let mut result = u64::MAX;

    let seeds: Vec<u64> = lines[0]
        .split_once(":")
        .expect("always have :")
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("always number"))
        .collect();

    let mut alamac: Vec<Vec<Vec<u64>>> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    for line in lines {
        if i == 0 {
            i += 1;
            continue;
        }
        if line
            .chars()
            .next()
            .expect("wil always have char")
            .is_alphabetic()
        {
            alamac.push(Vec::new());
            j += 1;
        } else {
            alamac[j - 1].push(
                line.split_whitespace()
                    .map(|x| x.parse::<u64>().expect("always number"))
                    .collect(),
            );
        }

        i += 1;
    }

    let num_threads: f64 = 100.0;
    for i in 0..seeds.len() / 2 {
        let number_of_seeds = seeds[i * 2 + 1];
        let chunk_size = (number_of_seeds as f64 / num_threads).ceil() as usize;
        let threads: Vec<_> = (0..num_threads.ceil() as usize)
            .map(|j| {
                let start = seeds[i * 2] + (j * chunk_size) as u64;
                let alamac_clone = alamac.clone();
                //println!("thread start");
                thread::spawn(move || {
                    let mut current_best = u64::MAX;

                    for seed in start..start + chunk_size as u64 {
                        let seed_number = seed;
                        let mut current_mapping = seed_number;
                        for maps in &alamac_clone {
                            for map in maps {
                                let length = map[2];
                                let destination_range_start = map[0];
                                let source_range_start = map[1];

                                if current_mapping < source_range_start + length
                                    && current_mapping >= source_range_start
                                {
                                    current_mapping = destination_range_start
                                        + (current_mapping - source_range_start);
                                    break;
                                }
                            }
                        }
                        if current_mapping < current_best {
                            current_best = current_mapping;
                        }
                    }
                    //println!("thread finished");
                    return current_best;
                })
            })
            .collect();

        for handle in threads {
            let res = handle.join().unwrap();
            if res < result {
                result = res;
            }
        }
    }

    result
}
