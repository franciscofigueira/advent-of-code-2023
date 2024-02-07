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
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (workflows, parts) = contents.split_once("\n\n").expect("always has the break");

    let workflows: Vec<_> = workflows.lines().collect();
    let parts: Vec<Part> = parts
        .lines()
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
        let valid = (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1);
        valid
    }
}
fn part_2(workflows_map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut result = 0;

    let mut instructions_range = vec![("in".to_string(), Part2::new())];

    while let Some((workflow_name, part)) = instructions_range.pop() {
        let workflow = workflows_map
            .get(workflow_name.as_str())
            .expect(format!("will exist {workflow_name}").as_str());
        let mut part = part.clone();

        for rule in workflow {
            if rule.contains(">") {
                let (parameter, remainder) = rule.split_once(">").expect("verified");
                let (value, destination) = remainder.split_once(":").expect("always have :");
                let value = value.parse::<usize>().expect("always number");
                let mut clone_part = part.clone();

                part.change_parameter_value(parameter, true, value);
                clone_part.change_parameter_value(parameter, false, value + 1);
                if destination == "R" {
                } else if destination == "A" {
                    result += clone_part.number_of_valid();
                } else {
                    instructions_range.push((destination.to_owned(), clone_part));
                }
            } else if rule.contains("<") {
                let (parameter, remainder) = rule.split_once("<").expect("verified");
                let (value, destination) = remainder.split_once(":").expect("always have :");
                let value = value.parse::<usize>().expect("always number");
                let mut clone_part = part.clone();

                part.change_parameter_value(parameter, false, value);
                clone_part.change_parameter_value(parameter, true, value - 1);
                if destination == "R" {
                } else if destination == "A" {
                    result += clone_part.number_of_valid();
                } else {
                    instructions_range.push((destination.to_owned(), clone_part));
                }
            } else {
                if rule == &"A" {
                    result += part.number_of_valid();
                } else if rule != &"R" {
                    instructions_range.push((rule.to_string(), part));
                }
            }
        }
    }

    result
}
