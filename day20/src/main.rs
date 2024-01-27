use std::{collections::HashMap, io::Read};

fn main() {
    let mut file = std::fs::File::open("./test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: HashMap<&str, (Vec<&str>, u8)> = HashMap::new();

    let lines: Vec<_> = contents
        .split("\r\n")
        .map(|x| {
            let (name, path) = x.split_once(" -> ").expect("aw");
            if name == "broadcaster" {
                map.insert(name, (path.split(",").collect(), 0));
            } else if name.contains("%") {
                map.insert(name.split_at(1).1, (path.split(",").collect(), 1));
            } else {
                map.insert(name.split_at(1).1, (path.split(",").collect(), 2));
            }
            return [name, path];
        })
        .collect();

    println!("{lines:?}");
    println!("{map:?}");
}
