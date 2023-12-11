use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<Vec<char>> = contents
        .split("\r\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut lines_to_expand: Vec<usize> = Vec::new();
    let mut rows_to_expand: Vec<usize> = Vec::new();
    for i in 0..lines.len() {
        if !lines[i].contains(&'#') {
            lines_to_expand.push(i);
        }
    }

    for i in 0..lines[0].len() {
        let mut expand = true;
        for j in 0..lines.len() {
            if lines[j][i] == '#' {
                expand = false;
                break;
            }
        }
        if expand {
            rows_to_expand.push(i);
        }
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let part_1_res = part_1(&galaxies, &lines_to_expand, &rows_to_expand);
    println!("{part_1_res:?}");
    let part_2_res = part_2(&galaxies, &lines_to_expand, &rows_to_expand);
    println!("{part_2_res:?}");
}

fn part_1(
    galaxies: &Vec<(usize, usize)>,
    lines_to_expand: &Vec<usize>,
    rows_to_expand: &Vec<usize>,
) -> usize {
    let mut result = 0;

    let mut total_galaxies_calculated: usize = 1;
    for i in 0..galaxies.len() - 1 {
        for j in total_galaxies_calculated..galaxies.len() {
            let mut galaxy_1 = galaxies[i].clone();
            let mut galaxy_2 = galaxies[j].clone();
            let distance = find_distance(
                &mut galaxy_1,
                &mut galaxy_2,
                &lines_to_expand,
                &rows_to_expand,
                2,
            );

            result += distance;
        }
        total_galaxies_calculated += 1;
    }

    result
}

fn part_2(
    galaxies: &Vec<(usize, usize)>,
    lines_to_expand: &Vec<usize>,
    rows_to_expand: &Vec<usize>,
) -> usize {
    let mut result = 0;

    let mut total_galaxies_calculated: usize = 1;
    for i in 0..galaxies.len() - 1 {
        for j in total_galaxies_calculated..galaxies.len() {
            let mut galaxy_1 = galaxies[i].clone();
            let mut galaxy_2 = galaxies[j].clone();
            let distance = find_distance(
                &mut galaxy_1,
                &mut galaxy_2,
                &lines_to_expand,
                &rows_to_expand,
                1000000,
            );

            result += distance;
        }
        total_galaxies_calculated += 1;
    }

    result
}

fn find_distance(
    galaxy_1: &mut (usize, usize),
    galaxy_2: &mut (usize, usize),
    lines_to_expand: &Vec<usize>,
    rows_to_expand: &Vec<usize>,
    expansion: usize,
) -> usize {
    let mut line_expansion = 0;
    let mut row_expansion = 0;
    if galaxy_1.1 > galaxy_2.1 {
        let temp = galaxy_1.1;
        galaxy_1.1 = galaxy_2.1;
        galaxy_2.1 = temp;
    }
    for line in lines_to_expand {
        if line > &galaxy_1.0 && line + line_expansion < galaxy_2.0 {
            galaxy_2.0 += expansion - 1;
            line_expansion += expansion - 1;
        }
        if line + line_expansion > galaxy_2.0 {
            break;
        }
    }
    for row in rows_to_expand {
        if row > &galaxy_1.1 && row + row_expansion < galaxy_2.1 {
            galaxy_2.1 += expansion - 1;
            row_expansion += expansion - 1;
        }
        if row + row_expansion > galaxy_2.1 {
            break;
        }
    }

    let distance = (galaxy_2.1 - galaxy_1.1) + (galaxy_2.0 - galaxy_1.0);
    distance
}
