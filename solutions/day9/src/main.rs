use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let num_vec = line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();
            input.push(num_vec);
        });
    input
}

fn get_neighbours(heightmap: &Vec<Vec<i32>>, x: &i32, y: &i32) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = vec![];
    for (xx, yy) in [(x, y - 1), (x, y + 1), (&(x - 1), *y), (&(x + 1), *y)] {
        if within_bounds(heightmap, &xx, &yy) {
            neighbours.push((*xx, yy));
        }
    }
    neighbours
}

fn within_bounds(heightmap: &Vec<Vec<i32>>, x: &i32, y: &i32) -> bool {
    *x >= 0 && *x < heightmap[0].len() as i32 && *y >= 0 && *y < heightmap.len() as i32
}

fn get_low_points(heightmap: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut low_points: Vec<(i32, i32)> = vec![];
    for y in 0..(heightmap.len()) as i32 {
        for x in 0..(heightmap[0].len()) as i32 {
            let neigbours = get_neighbours(heightmap, &x, &y);
            if neigbours.iter().all(|(nb_x, nb_y)| {
                heightmap[*nb_y as usize][*nb_x as usize] > heightmap[y as usize][x as usize]
            }) {
                low_points.push((x, y));
            }
        }
    }
    low_points
}

fn calculate_risk_level(height: &i32) -> i32 {
    height + 1
}

fn point_in_basin(basins: &Vec<Vec<(i32, i32)>>, x: i32, y: i32) -> bool {
    basins.iter().any(|bas| bas.contains(&(x, y)))
}

fn find_basin(heightmap: &Vec<Vec<i32>>, x: &i32, y: &i32) -> Option<Vec<(i32, i32)>> {
    let mut basin: Vec<(i32, i32)> = vec![];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((*x, *y));
    loop {
        let next = queue.pop_front();
        match next {
            Some((cur_x, cur_y)) => {
                if heightmap[cur_y as usize][cur_x as usize] < 9 {
                    basin.push((cur_x, cur_y));
                    let neighbours = get_neighbours(heightmap, &cur_x, &cur_y);
                    for nb in neighbours {
                        if !queue.contains(&nb) && !basin.contains(&nb) {
                            queue.push_back(nb);
                        }
                    }
                }
            }
            None => break,
        }
    }
    if basin.len() > 0 {
        Some(basin)
    } else {
        None
    }
}

fn find_basins(heightmap: &Vec<Vec<i32>>) -> Vec<Vec<(i32, i32)>> {
    let mut basins = vec![];
    for y in 0..(heightmap.len()) as i32 {
        for x in 0..(heightmap[0].len()) as i32 {
            if !point_in_basin(&basins, x, y) {
                let basin = find_basin(heightmap, &x, &y);
                match basin {
                    Some(b) => basins.push(b),
                    None => {}
                }
            }
        }
    }
    basins
}

fn part1(heightmap: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let low_points = get_low_points(heightmap);
    for l in low_points {
        let risk = calculate_risk_level(&heightmap[l.1 as usize][l.0 as usize]);
        result += risk;
    }
    result
}

fn part2(heightmap: &Vec<Vec<i32>>) -> i32 {
    let mut basins = find_basins(heightmap);
    let mut product = 1;
    basins.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
    for i in 0..3 {
        product *= basins[i].len();
    }
    product as i32
}

fn main() {
    let input = read_input_from_file("input.txt");
    let part1 = part1(&input);
    println!("Solution to part1: {}", part1);
    let part2 = part2(&input);
    println!("Solution to part2: {}", part2);
}
