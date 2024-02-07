use std::{collections::HashMap, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: [u32; 3],
    end: [u32; 3],
}

impl Brick {
    fn does_bricks_support(&self, other: &Self) -> bool {
        if self.end[2] != other.start[2] + 1 {
            return false;
        }
        self.start[0] >= other.start[0]
            && self.start[0] <= other.end[0]
            && self.end[1] >= other.start[1]
            && self.start[1] <= other.end[1]
    }
}
fn main() {
    let mut file = std::fs::File::open("./test_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut bricks = contents
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").expect("always have ~");
            let start: [u32; 3] = start
                .split(",")
                .map(|x| x.parse::<u32>().expect("always number"))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
            let end: [u32; 3] = end
                .split(",")
                .map(|x| x.parse::<u32>().expect("always number"))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();

            return Brick { start, end };
        })
        .collect::<Vec<Brick>>();

    bricks.sort_by(|a, b| a.start[2].cmp(&b.start[2]));

    // println!("{bricks:?}");

    let mut occupied_volume: HashMap<(u32, u32, u32), u32> = HashMap::new();

    let mut i = 1;
    for brick in &bricks {
        for x in brick.start[0]..(brick.end[0] + 1) {
            for y in brick.start[1]..(brick.end[1] + 1) {
                for z in brick.start[2]..(brick.end[2] + 1) {
                    occupied_volume.insert((x, y, z), i);
                }
            }
        }
        i += 1;
    }

    //let bricks fall
    // let mut supported_bricks: HashMap<u32, Vec<Brick>> = HashMap::new();
    // let mut supporter_bricks: HashMap<u32, Vec<Brick>> = HashMap::new();

    for brick in &mut bricks {
        //check if it can lower 1
        'outer: loop {
            let height = brick.start[2];
            if height == 1 {
                i += 1;
                break 'outer;
            }
            for x in brick.start[0]..(brick.end[0] + 1) {
                for y in brick.start[1]..(brick.end[1] + 1) {
                    if let Some(_) = occupied_volume.get(&(x, y, height - 1)) {
                        i += 1;
                        break 'outer;
                    }
                }
            }
            for x in brick.start[0]..(brick.end[0] + 1) {
                for y in brick.start[1]..(brick.end[1] + 1) {
                    occupied_volume.remove(&(x, y, brick.end[2]));
                    occupied_volume.insert((x, y, height - 1), i);
                }
            }
            brick.start[2] -= 1;
            brick.end[2] -= 1;
        }
        // if let Some(bricks) = supported_bricks.get_mut(&brick.start[2]) {
        //     bricks.push(brick.clone());
        // } else {
        //     supported_bricks.insert(brick.start[2], vec![brick.clone()]);
        // }
        // if let Some(bricks) = supporter_bricks.get_mut(&brick.start[2]) {
        //     bricks.push(brick.clone());
        // } else {
        //     supporter_bricks.insert(brick.end[2], vec![brick.clone()]);
        // }
    }

    let mut part_1_res = 0;
    'outer: for k in 0..bricks.len() {
        let mut occupied_volume_clone = occupied_volume.clone();
        let mut bricks_cloned = bricks.clone();
        let brick_to_remove = bricks_cloned.remove(k);
        for x in brick_to_remove.start[0]..(brick_to_remove.end[0] + 1) {
            for y in brick_to_remove.start[1]..(brick_to_remove.end[1] + 1) {
                for z in brick_to_remove.start[2]..(brick_to_remove.end[2] + 1) {
                    occupied_volume_clone.remove(&(x, y, z));
                }
            }
        }
        println!("brick that fall {bricks_cloned:?}");
        'next_b: for brick in &mut bricks_cloned {
            //check if it can lower 1
            loop {
                let height = brick.start[2];
                // if height == 1 {
                //     i += 1;
                //     continue 'outer;
                // }
                for x in brick.start[0]..(brick.end[0] + 1) {
                    for y in brick.start[1]..(brick.end[1] + 1) {
                        if let Some(_) = occupied_volume_clone.get(&(x, y, height - 1)) {
                            i += 1;
                            // println!("brick that fall {brick:?}");
                            part_1_res += 1;
                            continue 'outer;
                        }
                    }
                }
                continue 'next_b;
            }
        }
    }

    println!("{part_1_res:?}");
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct Brick {
//     brick: [[u32; 3]; 2],
// }

// fn main() {
//     let mut file = std::fs::File::open("./test_input.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();

//     let mut occupied_volume: HashMap<(u32, u32, u32), u32> = HashMap::new();
//     let mut bricks_on_level_0: HashMap<u32, Vec<Brick>> = HashMap::new();
//     let mut bricks_on_level_1: HashMap<u32, Vec<Brick>> = HashMap::new();

//     let mut bricks = contents
//         .lines()
//         .map(|line| {
//             let (start, end) = line.split_once("~").expect("always have ~");
//             let start: [u32; 3] = start
//                 .split(",")
//                 .map(|x| x.parse::<u32>().expect("always number"))
//                 .collect::<Vec<u32>>()
//                 .try_into()
//                 .unwrap();
//             let end: [u32; 3] = end
//                 .split(",")
//                 .map(|x| x.parse::<u32>().expect("always number"))
//                 .collect::<Vec<u32>>()
//                 .try_into()
//                 .unwrap();

//             return Brick {
//                 brick: [start, end],
//             };
//         })
//         .collect::<Vec<Brick>>();

//     bricks.sort_by(|a, b| a.brick[0][2].cmp(&b.brick[0][2]));

//     let mut i = 1;
//     for brick in &bricks {
//         let brick = brick.brick;
//         for x in brick[0][0]..(brick[1][0] + 1) {
//             for y in brick[0][1]..(brick[1][1] + 1) {
//                 for z in brick[0][2]..(brick[1][2] + 1) {
//                     occupied_volume.insert((x, y, z), i);
//                 }
//             }
//         }
//         i += 1;
//     }
//     let mut highest_level = 0;
//     let mut new_bricks: Vec<[[u32; 3]; 2]> = Vec::new();
//     i = 1;
//     for brick in &bricks {
//         let mut brick = brick.brick;
//         //check if it can lower 1
//         'outer: loop {
//             let height = brick[0][2];
//             if height == 1 {
//                 i += 1;
//                 break 'outer;
//             }
//             for x in brick[0][0]..(brick[1][0] + 1) {
//                 for y in brick[0][1]..(brick[1][1] + 1) {
//                     if let Some(_) = occupied_volume.get(&(x, y, height - 1)) {
//                         i += 1;
//                         break 'outer;
//                     }
//                 }
//             }
//             for x in brick[0][0]..(brick[1][0] + 1) {
//                 for y in brick[0][1]..(brick[1][1] + 1) {
//                     occupied_volume.remove(&(x, y, brick[1][2]));
//                     occupied_volume.insert((x, y, height - 1), i);
//                 }
//             }
//             brick[0][2] -= 1;
//             brick[1][2] -= 1;
//         }
//         new_bricks.push(brick);
//         if brick[1][2] > highest_level {
//             highest_level = brick[1][2];
//         }
//         if let Some(bricks) = bricks_on_level_0.get_mut(&brick[0][2]) {
//             bricks.push(Brick { brick });
//         } else {
//             bricks_on_level_0.insert(brick[0][2], vec![Brick { brick }]);
//         }
//         if let Some(bricks) = bricks_on_level_1.get_mut(&brick[1][2]) {
//             bricks.push(Brick { brick });
//         } else {
//             bricks_on_level_1.insert(brick[1][2], vec![Brick { brick }]);
//         }
//     }

//     let mut bricks_that_support: Vec<Brick> = Vec::new();

//     let mut part_1_result = 0;
//     for i in 2..(highest_level + 1) {
//         if let Some(bricks_above) = bricks_on_level_0.get(&i) {
//             let bricks_below = bricks_on_level_1
//                 .get(&(i - 1))
//                 .expect("always have bricks to support");
//             'next_brick: for j in 0..bricks_below.len() {
//                 let mut bricks_below_removed = bricks_below.clone();
//                 let removed_brick = bricks_below_removed.remove(j);

//                 for brick_above in bricks_above {
//                     let mut did_brick_fall = true;
//                     for brick_below in &bricks_below_removed {
//                         if brick_below.does_brick_support(brick_above) {
//                             did_brick_fall = false;
//                         }
//                     }
//                     if did_brick_fall {
//                         bricks_that_support.push(removed_brick);
//                         continue 'next_brick;
//                     }
//                 }
//                 part_1_result += 1;
//             }
//         }
//     }
//     part_1_result += bricks_on_level_1.get(&highest_level).unwrap().len();

//     println!("part_1_result {part_1_result:?}");

//     let mut part_2_res = 0;

//     'outer: for i in 0..bricks.len() {
//         let brick = Brick {
//             brick: new_bricks[i],
//         };
//         let mut num_that_fall = 0;
//         let mut bricks_on_level_1_c = bricks_on_level_1.clone();
//         let mut bricks_on_level_0_c = bricks_on_level_0.clone();

//         let mut bricks_to_remove: Vec<Brick> = Vec::new();
//         bricks_to_remove.push(brick.clone());
//         for j in (brick.brick[1][2] + 1)..highest_level {
//             let brick_vec = bricks_on_level_0_c.get_mut(&(j - 1)).unwrap();
//             let brick_vec_removed: Vec<Brick> = brick_vec
//                 .iter()
//                 .filter_map(|b| {
//                     for b1 in &bricks_to_remove {
//                         if b1 == b {
//                             return None;
//                         }
//                     }
//                     Some(b.clone())
//                 })
//                 .collect();
//             // println!("{brick_vec_removed:?}");
//             bricks_to_remove.clear();
//             if let Some(above_bricks) = bricks_on_level_0_c.get(&j) {
//                 //let mut did_any_brick_fall = false;
//                 for above_brick in above_bricks {
//                     let mut did_brick_fall = true;
//                     for brick_below in &brick_vec_removed {
//                         if brick_below.does_brick_support(above_brick) {
//                             did_brick_fall = false;
//                             //did_any_brick_fall = true;
//                             bricks_to_remove.push(above_brick.clone());
//                         }
//                     }
//                     if did_brick_fall {
//                         println!("{above_brick:?}");
//                         num_that_fall += 1;
//                         part_2_res += 1;
//                     }
//                 }
//                 // if did_any_brick_fall == false {
//                 //     break 'outer;
//                 // }
//             }
//         }
//     }

//     println!("part_2_result {part_2_res:?}");
//     //println!("support {bricks_that_support:?}");
// }

// impl Brick {
//     fn does_brick_support(&self, other: &Self) -> bool {
//         let mut x: bool = false;
//         if self.brick[0][0] >= other.brick[0][0] {
//             if self.brick[0][0] <= other.brick[1][0] {
//                 x = true;
//             }
//         } else {
//             if self.brick[1][0] >= other.brick[0][0] {
//                 x = true;
//             }
//         }

//         let mut y: bool = false;

//         if self.brick[0][1] >= other.brick[0][1] {
//             if self.brick[0][1] <= other.brick[1][1] {
//                 y = true;
//             }
//         } else {
//             if self.brick[1][1] >= other.brick[0][1] {
//                 y = true;
//             }
//         }

//         if x && y {
//             return true;
//         }
//         return false;
//         // self.brick[1][0] >= other.brick[0][0]
//         //     && self.brick[0][0] <= other.brick[1][0]
//         //     && self.brick[1][1] >= other.brick[0][1]
//         //     && self.brick[0][1] <= other.brick[1][1]
//     }
// }

// #[test]
// fn test_brick_support() {
//     let a = Brick {
//         brick: [[0, 0, 1], [0, 1, 1]],
//     };
//     let b = Brick {
//         brick: [[1, 1, 1], [1, 1, 1]],
//     };
//     let c = Brick {
//         brick: [[0, 0, 2], [0, 0, 2]],
//     };

//     assert!(!c.does_brick_support(&b));
//     assert!(!b.does_brick_support(&c));
//     // assert!(!b.does_brick_support(&c));
//     // assert!(!c.does_brick_support(&b));
// }
