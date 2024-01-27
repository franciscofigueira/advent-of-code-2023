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

fn part_2_not_brute_force(lines: &mut Vec<&str>) -> u64 {
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

    let mut current_ranges: Vec<Vec<u64>> = Vec::new();
    let mut new_ranges: Vec<Vec<u64>> = Vec::new();

    for i in 0..seeds.len() / 2 {
        current_ranges.push([seeds[i * 2], seeds[i * 2 + 1]].to_vec());
    }
    println!("{current_ranges:?}");
    println!("{alamac:?}");

    // for i in 0..alamac.len() {
    //     for range in &current_ranges {
    //         let mut start = range[0];
    //         let mut len = range[1];

    //         for map in &alamac[i] {
    //             let destination_range_start = map[0];
    //             let source_range_start = map[1];
    //             let map_length = map[2];

    //             if source_range_start >= start && source_range_start + map_length <= start {
    //                 if start + len <= source_range_start + map_length {
    //                     new_ranges.push(vec![
    //                         destination_range_start + (start - source_range_start),
    //                         len,
    //                     ]);
    //                     break;
    //                 } else if start + len >= source_range_start && {
    //                     new_ranges.push(vec![
    //                         destination_range_start + (start - source_range_start),
    //                         source_range_start - len,
    //                     ]);
    //                 }
    //             }

    //         }
    //     }

    //     current_ranges = new_ranges.clone();
    // }

    // for i in 0..alamac.len() {
    //     for j in 0..current_range.len() / 2 {
    //         let mut start_range = current_range[j * 2];
    //         let mut range_len = current_range[j * 2 + 1];

    //         for map in &alamac[i] {
    //             let length = map[2];
    //             let destination_range_start = map[0];
    //             let source_range_start = map[1];
    //             if start_range > source_range_start
    //                 && source_range_start + length > start_range + range_len
    //             {
    //                 new_range.push(
    //                     destination_range_start - (start_range - (source_range_start + length)),
    //                 );
    //                 new_range.push(start_range + length - (source_range_start + length));
    //             }
    //         }
    //         current_range = new_range.clone();
    //     }
    // }
    // println!("{current_range:?}");

    // for i in 0..seeds.len() / 2 {
    //     let mut current_mapping = u64::MAX;
    //     let seed_start_range = seeds[i * 2].parse::<u64>().expect("always number");
    //     let seed_range = seeds[i * 2 + 1].parse::<u64>().expect("always number");

    //     let mut current_range_start = seed_start_range;
    //     let mut current_range_len = seed_range;

    //     let mut new_ranges = Vec::new();
    //     for maps in &alamac {
    //         for map in maps {
    //             let length = map[2].parse::<u64>().expect("always number");
    //             let destination_range_start = map[0].parse::<u64>().expect("always number");
    //             let source_range_start = map[1].parse::<u64>().expect("always number");

    //             if current_range_start >= source_range_start
    //                 && current_range_start <= source_range_start + length
    //             {}
    //         }
    //     }
    //     if current_mapping < result {
    //         result = current_mapping;
    //     }
    // }

    result
}
