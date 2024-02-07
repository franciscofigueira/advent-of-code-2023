use std::io::Read;

#[derive(Debug)]
struct Particle {
    pos: [f64; 3],
    vel: [f64; 3],
}

impl Particle {
    fn particle_intersection_2d(&self, other: &Self) -> Option<(f64, f64)> {
        //parallel
        if self.vel[0] / other.vel[0] == self.vel[1] / other.vel[1] {
            return None;
        }
        let t = ((self.pos[1] - other.pos[1]) * other.vel[0]
            - (self.pos[0] - other.pos[0]) * other.vel[1])
            / (-self.vel[1] * other.vel[0] + self.vel[0] * other.vel[1]);

        let u = (self.pos[0] + self.vel[0] * t - other.pos[0]) / other.vel[0];
        if t < 0.0 || u < 0.0 {
            return None;
        }

        Some((
            self.get_position_in_time(t)[0],
            self.get_position_in_time(t)[1],
        ))
    }

    fn get_position_in_time(&self, time: f64) -> [f64; 3] {
        return [
            self.pos[0] + self.vel[0] * time,
            self.pos[1] + self.vel[1] * time,
            self.pos[1] + self.vel[1] * time,
        ];
    }
}

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<_> = contents
        .lines()
        .map(|x| {
            let (positions, velocities) = x.split_once("@").expect("always contains @");
            let positions: Vec<_> = positions
                .split(",")
                .map(|x| {
                    x.split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f64>()
                        .expect("number")
                })
                .collect();
            let velocities: Vec<_> = velocities
                .split(",")
                .map(|x| {
                    x.split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f64>()
                        .expect("number")
                })
                .collect();

            return Particle {
                pos: [positions[0], positions[1], positions[2]],
                vel: [velocities[0], velocities[1], velocities[2]],
            };
        })
        .collect();
    //println!("{lines:?}");

    let part_1_res = part_1(&lines, 200000000000000.0, 400000000000000.0);
    println!("part 1 res: {part_1_res:?}");
}

fn part_1(hails: &Vec<Particle>, limit_start: f64, limit_end: f64) -> usize {
    let mut result = 0;

    for i in 0..hails.len() {
        let hail_a = hails.get(i).unwrap();
        for j in (i + 1)..hails.len() {
            let hail_b = hails.get(j).unwrap();
            if let Some((x, y)) = hail_a.particle_intersection_2d(hail_b) {
                if x >= limit_start && y >= limit_start && x <= limit_end && y <= limit_end {
                    result += 1;
                }
            }
        }
    }

    result
}

// part 2, this is a simple problem with 9 equations and 9 variables see notes.md for more information
#[test]
fn test_point_intersection() {
    let part_a = Particle {
        pos: [19.0, 13.0, 30.0],
        vel: [-2.0, 1.0, -2.0],
    };
    let part_b = Particle {
        pos: [12.0, 31.0, 28.0],
        vel: [-1.0, -2.0, -1.0],
    };

    let t = part_a.particle_intersection_2d(&part_b);
    println!("{t:?}");
}
