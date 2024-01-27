use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

#[derive(Hash, Clone, Debug, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Part {
    fn sum(&self) -> usize {
        let sum = self.x + self.m + self.a + self.s;
        sum
    }

    fn get_parameter_value(&self, parameter: &str) -> usize {
        match parameter {
            "x" => {
                return self.x;
            }
            "m" => {
                return self.m;
            }
            "a" => {
                return self.a;
            }
            "s" => {
                return self.s;
            }
            _ => {
                return 0;
            }
        }
    }
}

fn main() {
    let mut file = std::fs::File::open("./test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (workflows, parts) = contents
        .split_once("\r\n\r\n")
        .expect("always has the break");

    let workflows: Vec<_> = workflows.split("\r\n").collect();
    let parts: Vec<Part> = parts
        .split("\r\n")
        .map(|x| {
            let attributes: Vec<_> = x.split(",").collect();
            let x = attributes[0]
                .split_once("=")
                .expect("always have =")
                .1
                .parse::<usize>()
                .expect("awlays number");
            let m = attributes[1]
                .split_once("=")
                .expect("always have =")
                .1
                .parse::<usize>()
                .expect("awlays number");
            let a = attributes[2]
                .split_once("=")
                .expect("always have =")
                .1
                .parse::<usize>()
                .expect("awlays number");

            let s = attributes[3]
                .split_once("=")
                .expect("always have =")
                .1
                .split_once("}")
                .expect("awlywa have }")
                .0
                .parse::<usize>()
                .expect("awlays number");

            return Part { x, m, a, s };
        })
        .collect();
    let mut workflows_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for instruction in workflows {
        let (name, rules) = instruction.split_once("{").expect("always have {");
        let rules = rules.split_once("}").expect("always have }").0;
        let rules = rules.split(",").collect();
        workflows_map.insert(name, rules);
    }

    let part_1_result = part_1(&workflows_map, &parts);
    println!(" part 1 result: {part_1_result:?}");
    let part_2_result = part_2(&workflows_map);
    println!(" part 2 result: {part_2_result:?}");
}

fn part_1(workflows_map: &HashMap<&str, Vec<&str>>, parts: &Vec<Part>) -> usize {
    let mut result = 0;

    let initial_workflow = workflows_map.get("in").expect("always exists");

    for part in parts {
        let mut current_workflow = initial_workflow;

        'outer: loop {
            for rule in current_workflow {
                if rule == &"R" {
                    break 'outer;
                } else if rule == &"A" {
                    result += part.sum();
                    break 'outer;
                } else if rule.contains(">") {
                    let (parameter, remainder) = rule.split_once(">").expect("verified");
                    let (value, destination) = remainder.split_once(":").expect("always have :");
                    let value = value.parse::<usize>().expect("always number");
                    if part.get_parameter_value(parameter) > value {
                        if destination == "A" {
                            result += part.sum();
                            break 'outer;
                        } else if destination == "R" {
                            break 'outer;
                        } else {
                            current_workflow =
                                workflows_map.get(destination).expect("always exists");
                            break;
                        }
                    }
                } else if rule.contains("<") {
                    let (parameter, remainder) = rule.split_once("<").expect("verified");
                    let (value, destination) = remainder.split_once(":").expect("always have :");
                    let value = value.parse::<usize>().expect("always number");
                    if part.get_parameter_value(parameter) < value {
                        if destination == "A" {
                            result += part.sum();
                            break 'outer;
                        } else if destination == "R" {
                            break 'outer;
                        } else {
                            current_workflow =
                                workflows_map.get(destination).expect("always exists");
                            break;
                        }
                    }
                } else {
                    current_workflow = workflows_map.get(rule).expect("always exists");
                    break;
                }
            }
        }
    }

    result
}

#[derive(Hash, Clone, Debug, Copy)]
struct Part2 {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}
impl Part2 {
    const MAX_RATING: usize = 4000;
    const MIN_RATING: usize = 1;

    fn new() -> Self {
        Part2 {
            x: (Self::MIN_RATING, Self::MAX_RATING),
            m: (Self::MIN_RATING, Self::MAX_RATING),
            a: (Self::MIN_RATING, Self::MAX_RATING),
            s: (Self::MIN_RATING, Self::MAX_RATING),
        }
    }
    fn get_parameter_range(&self, parameter: &str) -> (usize, usize) {
        match parameter {
            "x" => {
                return self.x;
            }
            "m" => {
                return self.m;
            }
            "a" => {
                return self.a;
            }
            "s" => {
                return self.s;
            }
            _ => {
                return (0, 0);
            }
        }
    }
    fn change_parameter_value(&mut self, parameter: &str, is_max: bool, new_value: usize) {
        match parameter {
            "x" => {
                if is_max {
                    self.x.1 = new_value;
                } else {
                    self.x.0 = new_value;
                }
            }
            "m" => {
                if is_max {
                    self.m.1 = new_value;
                } else {
                    self.m.0 = new_value;
                }
            }
            "a" => {
                if is_max {
                    self.a.1 = new_value;
                } else {
                    self.a.0 = new_value;
                }
            }
            "s" => {
                if is_max {
                    self.s.1 = new_value;
                } else {
                    self.s.0 = new_value;
                }
            }
            _ => {}
        }
    }

    fn number_of_valid(&self) -> usize {
        let valid = (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0);
        valid
    }
}
fn part_2(workflows_map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut result = 0;

    let mut instructions_range: Vec<(String, Part2)> = Vec::new();
    let initial_workflow = workflows_map.get("in").expect("always exists");

    let mut initial_part = Part2::new();
    for rule in initial_workflow {
        if rule.contains(">") {
            let (parameter, remainder) = rule.split_once(">").expect("verified");
            let (value, destination) = remainder.split_once(":").expect("always have :");
            let value = value.parse::<usize>().expect("always number");
            let mut clone_part = initial_part.clone();
            let range = initial_part.get_parameter_range(parameter);
            if range.0 > value && range.1 < value {
                initial_part.change_parameter_value(parameter, false, value);
                clone_part.change_parameter_value(parameter, true, value);
                instructions_range.push((destination.to_owned(), clone_part));
            } else {
                initial_part.change_parameter_value(parameter, true, value);
                clone_part.change_parameter_value(parameter, false, value);
                instructions_range.push((destination.to_owned(), clone_part));
            }
        } else if rule.contains("<") {
            let (parameter, remainder) = rule.split_once("<").expect("verified");
            let (value, destination) = remainder.split_once(":").expect("always have :");
            let value = value.parse::<usize>().expect("always number");
            let mut clone_part = initial_part.clone();
            let range = initial_part.get_parameter_range(parameter);
            if range.0 < value && range.1 > value {
                initial_part.change_parameter_value(parameter, false, value);
                clone_part.change_parameter_value(parameter, true, value);
                instructions_range.push((destination.to_owned(), clone_part));
            } else {
                initial_part.change_parameter_value(parameter, true, value);
                clone_part.change_parameter_value(parameter, false, value);
                instructions_range.push((destination.to_owned(), clone_part));
            }
        } else {
            //current_workflow = workflows_map.get(rule).expect("always exists");
            instructions_range.push((rule.to_string(), initial_part));
            break;
        }
    }

    println!("{instructions_range:?}");

    while instructions_range.len() != 0 {
        'outer: for i in 0..instructions_range.len() {
            if i >= instructions_range.len() - 1 {
                continue;
            }
            //println!("{instructions_range:?}");
            println!("{result:?}");
            let instruction = &mut instructions_range[i];
            let mut new_inst: Vec<(String, Part2)> = Vec::new();
            let workflow = workflows_map
                .get(instruction.0.as_str())
                .expect("always exists");
            for rule in workflow {
                if rule == &"R" {
                    instructions_range.swap_remove(i);
                    break 'outer;
                } else if rule == &"A" {
                    result += instruction.1.clone().number_of_valid();
                    instructions_range.swap_remove(i);

                    break 'outer;
                } else if rule.contains(">") {
                    let (parameter, remainder) = rule.split_once(">").expect("verified");
                    let (value, destination) = remainder.split_once(":").expect("always have :");
                    let value = value.parse::<usize>().expect("always number");
                    let mut clone_part = instruction.1.clone();
                    let range = instruction.1.get_parameter_range(parameter);
                    if range.0 > value && range.1 < value {
                        instruction
                            .1
                            .change_parameter_value(parameter, false, value);

                        clone_part.change_parameter_value(parameter, true, value);

                        new_inst.push((destination.to_owned(), clone_part));
                    } else {
                        instruction.1.change_parameter_value(parameter, true, value);
                        clone_part.change_parameter_value(parameter, false, value);
                        new_inst.push((destination.to_owned(), clone_part));
                    }
                } else if rule.contains("<") {
                    let (parameter, remainder) = rule.split_once("<").expect("verified");
                    let (value, destination) = remainder.split_once(":").expect("always have :");
                    let value = value.parse::<usize>().expect("always number");
                    let mut clone_part = instruction.1.clone();
                    let range = instruction.1.get_parameter_range(parameter);
                    if range.0 < value && range.1 > value {
                        instruction
                            .1
                            .change_parameter_value(parameter, false, value);
                        clone_part.change_parameter_value(parameter, true, value);
                        new_inst.push((destination.to_owned(), clone_part));
                    } else {
                        instruction.1.change_parameter_value(parameter, true, value);
                        clone_part.change_parameter_value(parameter, false, value);
                        new_inst.push((destination.to_owned(), clone_part));
                    }
                } else {
                    //current_workflow = workflows_map.get(rule).expect("always exists");

                    new_inst.push((rule.to_string(), instruction.1));
                    break;
                }
            }
            instructions_range.extend(new_inst);
        }
    }

    result
}
