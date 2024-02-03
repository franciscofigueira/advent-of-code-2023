use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    io::Read,
};

#[derive(Debug, Clone)]
enum Module {
    Broadcast,
    Conjunction(HashMap<String, bool>),
    FlipFlop(bool),
}

impl Module {
    fn output(&mut self, input: bool, from: &str) -> Option<bool> {
        match self {
            Self::Conjunction(previous_inputs) => {
                previous_inputs.insert(from.to_string(), input);
                for value in previous_inputs.values() {
                    if value == &false {
                        return Some(true);
                    }
                }

                Some(false)
            }
            Self::FlipFlop(state) => {
                if input {
                    None
                } else {
                    if *state == true {
                        *state = false;
                        Some(false)
                    } else {
                        *state = true;
                        Some(true)
                    }
                }
            }
            Self::Broadcast => Some(input),
        }
    }
}

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: HashMap<&str, (Vec<&str>, Module)> = HashMap::new();
    let mut copy_destination: HashMap<&str, Vec<&str>> = HashMap::new();

    let _lines: Vec<_> = contents
        .lines()
        .map(|x| {
            let (name, path) = x.split_once(" -> ").expect("aw");
            if name == "broadcaster" {
                map.insert(name, (path.split(", ").collect(), Module::Broadcast));
                copy_destination.insert(name, path.split(", ").collect());
            } else if name.contains("%") {
                map.insert(
                    name.split_at(1).1,
                    (path.split(", ").collect(), Module::FlipFlop(false)),
                );
                copy_destination.insert(name, path.split(", ").collect());
            } else {
                map.insert(
                    name.split_at(1).1,
                    (
                        path.split(", ").collect(),
                        Module::Conjunction(HashMap::new()),
                    ),
                );
                copy_destination.insert(name, path.split(", ").collect());
            }
            return [name, path];
        })
        .collect();

    for (key, dests) in copy_destination.iter() {
        for dest in dests {
            if let Some((_, module)) = map.get_mut(dest) {
                match module {
                    Module::Conjunction(state) => {
                        state.insert(key.split_at(1).1.to_string(), false);
                    }
                    _ => {}
                }
            }
        }
    }

    // println!("{map:?}");
    let part_1_res = part_1(&mut map.clone(), 1000);
    println!("part 1 res: {part_1_res}");
    let part_2_res = part_2(&mut map);
    println!("part 2 res: {part_2_res}");
}

fn part_1(map: &mut HashMap<&str, (Vec<&str>, Module)>, number_button_pushes: usize) -> usize {
    let mut number_low_pulses = 0;
    let mut number_high_pulses = 0;

    let mut active_signals: VecDeque<(&str, &str, bool)> = VecDeque::new();

    for _ in 0..number_button_pushes {
        number_low_pulses += 1;

        for recipient in map.get("broadcaster").expect("always have").0.clone() {
            active_signals.push_back((recipient, "broadcaster", false));
            number_low_pulses += 1;
        }

        'outer: loop {
            let signal = active_signals.pop_front();
            if let Some((dest, source, input)) = signal {
                if let Some((recipients, module)) = map.get_mut(dest) {
                    let output = module.output(input, source);

                    if let Some(output) = output {
                        for recipient in recipients {
                            active_signals.push_back((recipient, dest, output));
                            if output {
                                number_high_pulses += 1;
                            } else {
                                number_low_pulses += 1;
                            }
                        }
                    }
                }
            } else {
                break 'outer;
            }
        }
    }
    let result = number_high_pulses * number_low_pulses;

    result
}

fn part_2(map: &mut HashMap<&str, (Vec<&str>, Module)>) -> usize {
    let mut counter = 0;

    let mut active_signals: VecDeque<(&str, &str, bool)> = VecDeque::new();

    let mut earliest_signal: HashMap<String, u64> = HashMap::new();

    //last signal rx depends on output of zp which is conjuction module
    // so we need to check lcm of all inputs
    match &map.get("zp").expect("exists").1 {
        Module::Conjunction(m) => {
            for key in m.keys() {
                earliest_signal.insert(key.clone(), 0);
            }
        }
        _ => {}
    }
    counter += 1;

    'outer: loop {
        for recipient in map.get("broadcaster").expect("always have").0.clone() {
            active_signals.push_back((recipient, "broadcaster", false));
        }

        'inner: loop {
            let signal = active_signals.pop_front();
            if let Some((dest, source, input)) = signal {
                if dest == "zp" && input == true {
                    let current_value = earliest_signal.get(source).expect("exists");
                    if *current_value == 0 {
                        earliest_signal.insert(source.to_string(), counter);
                    }
                    let mut all_set = true;
                    for value in earliest_signal.values() {
                        if value == &0 {
                            all_set = false;
                        }
                    }
                    if all_set {
                        break 'outer;
                    }
                }
                if let Some((recipients, module)) = map.get_mut(dest) {
                    let output = module.output(input, source);

                    if let Some(output) = output {
                        for recipient in recipients {
                            active_signals.push_back((recipient, dest, output));
                        }
                    }
                }
            } else {
                break 'inner;
            }
        }
        counter += 1;
    }

    let mut v = earliest_signal
        .values()
        .map(|x| x.to_owned() as i64)
        .collect::<Vec<i64>>();

    let mut result = lcm(
        v.pop().expect("will have") as i64,
        v.pop().expect("will have") as i64,
    );
    while v.len() != 0 {
        result = lcm(result, v.pop().expect("always have"));
    }

    return result as usize;
}

#[test]
fn test_module() {
    let mut flipflop = Module::FlipFlop(false);

    let outptut = flipflop.output(false, "");

    assert_eq!(outptut, Some(true));
    match flipflop {
        Module::FlipFlop(state) => {
            assert_eq!(state, true);
        }
        _ => {}
    }
}
