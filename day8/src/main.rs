use num::integer::lcm;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r\n").collect();

    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut i = 0;
    let instructions = lines[0];
    for line in lines {
        if i < 2 {
            i += 1;
            continue;
        }
        let (node, paths) = line.split_once(" = ").expect("always has =");
        let mut paths = paths.chars();
        paths.next();
        paths.next_back();
        let (left_path, right_path) = paths.as_str().split_once(", ").expect("always have");
        nodes.insert(node, [left_path, right_path]);
    }

    let part_1_res = part_1(&nodes, instructions);
    println!("{part_1_res:?}");
    let part_2_res = part_2(&nodes, instructions);
    println!("{part_2_res:?}");
}

fn part_1(nodes: &HashMap<&str, [&str; 2]>, instructions: &str) -> u64 {
    let mut result = 0;

    let mut current_node = "AAA";
    let mut current_node_paths = nodes.get(current_node).expect("always has path");

    let mut instructions_copy = instructions.chars();
    while current_node != "ZZZ" {
        if let Some(instruction) = instructions_copy.next() {
            if instruction == 'L' {
                current_node = current_node_paths[0];
                current_node_paths = nodes.get(current_node).expect("always has path");
            } else {
                current_node = current_node_paths[1];
                current_node_paths = nodes.get(current_node).expect("always has path");
            }
        } else {
            instructions_copy = instructions.chars();
            continue;
        }

        result += 1;
    }
    result
}

fn part_2(nodes: &HashMap<&str, [&str; 2]>, instructions: &str) -> u64 {
    let mut result = 0;

    let mut nodes_iter = nodes.iter();
    let mut current_nodes: Vec<&str> = Vec::new();
    let mut current_paths: Vec<&[&str; 2]> = Vec::new();
    let mut num_steps_z: Vec<u64> = Vec::new();
    while let Some(node) = nodes_iter.next() {
        if node.0.chars().next_back().expect("always char") == 'A' {
            current_nodes.push(node.0);
            current_paths.push(node.1);
            num_steps_z.push(0);
        }
    }
    let mut instructions_copy = instructions.chars();
    while true {
        if let Some(instruction) = instructions_copy.next() {
            for i in 0..current_nodes.len() {
                if instruction == 'L' {
                    current_nodes[i] = current_paths[i][0];
                    current_paths[i] = nodes.get(current_nodes[i]).expect("always has node");
                } else {
                    current_nodes[i] = current_paths[i][1];
                    current_paths[i] = nodes.get(current_nodes[i]).expect("always has node");
                }
                if current_nodes[i]
                    .chars()
                    .next_back()
                    .expect("always have char")
                    == 'Z'
                {
                    if num_steps_z[i] == 0 {
                        num_steps_z[i] = result + 1;
                    }
                }
            }

            let mut is_path_found = true;
            result += 1;
            for i in 0..num_steps_z.len() {
                if num_steps_z[i] == 0 {
                    is_path_found = false;
                }
            }
            if is_path_found {
                break;
            }
        } else {
            instructions_copy = instructions.chars();
            continue;
        }
    }

    let mut current_r = lcm(
        num_steps_z.pop().expect("has number") as i64,
        num_steps_z.pop().expect("has number") as i64,
    );
    while num_steps_z.len() != 0 {
        current_r = lcm(current_r, num_steps_z.pop().expect("has number") as i64);
    }

    result = current_r as u64;
    result
}
