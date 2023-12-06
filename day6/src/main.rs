use std::io::Read;

const ACCELERATION: f64 = 1.0; //1mm/s^2

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r\n").collect();
    let (_, times) = lines[0].split_once(":").expect("always has :");
    let times_as_f64: Vec<f64> = times
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("always number"))
        .collect();
    let (_, distances) = lines[1].split_once(":").expect("always has :");
    let distances_as_f64: Vec<f64> = distances
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("always number"))
        .collect();

    let part_1_res = part_1(&times_as_f64, &distances_as_f64);
    println!("part 1 result: {part_1_res}");
    let part_2_res = part_2(times, distances);
    println!("part 2 result: {part_2_res}");
}

// x = v0 * t
// v0 = a * t0
// t0 + t = T
// x > D

//a*t0*T - t0^2 -D > 0
fn part_1(times: &Vec<f64>, distances: &Vec<f64>) -> u64 {
    let mut result: u64 = 1;

    let mut i = 0;
    for time in times {
        let t_0_1 = (-ACCELERATION * time
            + (((ACCELERATION * time).powf(2.0) - 4.0 * (distances[i] + 0.1)) as f64).sqrt())
            / 2.0
            * (-1.0);
        let t_0_2 = (-ACCELERATION * time
            - (((ACCELERATION * time).powf(2.0) - 4.0 * (distances[i] + 0.1)) as f64).sqrt())
            / 2.0
            * (-1.0);

        let number_of_ways = t_0_2.floor() as u64 - t_0_1.ceil() as u64 + 1;

        result *= number_of_ways;
        i += 1;
    }

    result
}

fn part_2(time: &str, distance: &str) -> u64 {
    let time: Vec<_> = time.split_whitespace().collect();
    let time = time.join("").parse::<f64>().unwrap();

    let distance: Vec<_> = distance.split_whitespace().collect();
    let distance = distance.join("").parse::<f64>().unwrap();

    let t_0_1 = (-ACCELERATION * time
        + (((ACCELERATION * time).powf(2.0) - 4.0 * (distance + 0.1)) as f64).sqrt())
        / 2.0
        * (-1.0);
    let t_0_2 = (-ACCELERATION * time
        - (((ACCELERATION * time).powf(2.0) - 4.0 * (distance + 0.1)) as f64).sqrt())
        / 2.0
        * (-1.0);

    let number_of_ways = t_0_2.floor() as u64 - t_0_1.ceil() as u64 + 1;

    number_of_ways
}
