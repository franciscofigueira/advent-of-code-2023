use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r").collect();

    let part_1_res = part_1(&lines, 12, 13, 14);
    let part_2_res = part_2(&lines);
    println!("part 1 result: {part_1_res}");
    println!("part 2 result: {part_2_res}");
}

fn part_1(lines: &Vec<&str>, n_red: u32, n_green: u32, n_blue: u32) -> u32 {
    let mut sum_possible_games = 0;
    let mut i = 1;
    for line in lines {
        let (_, game) = line.split_once(":").unwrap();
        let mut valid = true;
        for set in game.split(';') {
            let set_divided: Vec<_> = set.split(',').collect();

            let mut game_number_red = 0;
            let mut game_number_green = 0;
            let mut game_number_blue = 0;

            for outcomes in set_divided {
                let number_color: Vec<_> = outcomes.split_whitespace().collect();

                if number_color[1] == "red" {
                    game_number_red += number_color[0].parse::<u32>().unwrap();
                }
                if number_color[1] == "green" {
                    game_number_green += number_color[0].parse::<u32>().unwrap();
                }

                if number_color[1] == "blue" {
                    game_number_blue += number_color[0].parse::<u32>().unwrap();
                }
            }
            if game_number_red > n_red || game_number_green > n_green || game_number_blue > n_blue {
                valid = false;
                break;
            }
        }
        if valid {
            sum_possible_games += i;
        }

        i += 1;
    }

    sum_possible_games
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut sum_power_set_cubes = 0;

    for line in lines {
        let (_, game) = line.split_once(":").unwrap();

        let mut lowest_number_red = 0;
        let mut lowest_number_green = 0;
        let mut lowest_number_blue = 0;
        for set in game.split(';') {
            let set_divided: Vec<_> = set.split(',').collect();

            for outcomes in set_divided {
                let number_color: Vec<_> = outcomes.split_whitespace().collect();
                let num = number_color[0].parse::<u32>().unwrap();
                if number_color[1] == "red" {
                    if num > lowest_number_red {
                        lowest_number_red = num;
                    }
                }
                if number_color[1] == "green" {
                    if num > lowest_number_green {
                        lowest_number_green = num;
                    }
                }

                if number_color[1] == "blue" {
                    if num > lowest_number_blue {
                        lowest_number_blue = num;
                    }
                }
            }
        }
        sum_power_set_cubes += lowest_number_red * lowest_number_green * lowest_number_blue;
    }
    sum_power_set_cubes
}
