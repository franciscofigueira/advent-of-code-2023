use std::io::Read;

#[derive(Debug)]
struct Particle {
    pos: [f64; 3],
    vel: [f64; 3],
}

impl Particle {
    fn particle_intersection_2d(&self, other: &Self) -> Option<f64> {
        let a = self.vel[0].powf(2.0)
            + other.vel[0] * (other.vel[0] - 2.0 * self.vel[0])
            + self.vel[1].powf(2.0)
            + other.vel[1] * (other.vel[1] - 2.0 * self.vel[1]);
        let b = self.vel[0] * (2.0 * self.pos[0] - 2.0 * other.pos[0])
            + other.vel[0] * (2.0 * other.pos[0] - 2.0 * self.pos[0])
            + self.vel[1] * (2.0 * self.pos[1] - 2.0 * other.pos[1])
            + other.vel[1] * (2.0 * other.pos[1] - 2.0 * self.pos[1]);
        let c = self.pos[0] * (self.pos[0] - 2.0 * other.pos[0])
            + other.pos[0].powf(2.0)
            + self.pos[1] * (self.pos[0] - 2.0 * other.pos[1])
            + other.pos[1].powf(2.0);

        let inside_sqrt = b.powf(2.0) - 4.0 * a * c;
        println!("{inside_sqrt}");
        if inside_sqrt < 0.0 {
            return None;
        } else {
            let t1 = (-b + (inside_sqrt).sqrt()) / 2.0 * a;
            let t2 = (-b - (inside_sqrt).sqrt()) / 2.0 * a;
            if t1 < 0.0 && t2 < 0.0 {
                return None;
            } else {
                if t1 < t2 && t1 > 0.0 {
                    return Some(t1);
                } else {
                    return Some(t2);
                }
            }
        }
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
    let mut file = std::fs::File::open("./test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<_> = contents
        .split("\r\n")
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
    println!("{lines:?}");

    let part_1_res = part_1(&lines);
}

fn part_1(hails: &Vec<Particle>) -> usize {
    let mut result = 0;

    result
}

#[test]
fn test_point_intersection() {
    let partA = Particle {
        pos: [19.0, 13.0, 30.0],
        vel: [-2.0, 1.0, -2.0],
    };
    let partB = Particle {
        pos: [18.0, 19.0, 22.0],
        vel: [-1.0, -1.0, -2.0],
    };

    let t = partA.particle_intersection_2d(&partB);
    println!("{t:?}");
}
