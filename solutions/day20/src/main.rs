use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct ImageProblem {
    key: String,
    points: HashSet<(i32, i32)>,
}

impl ImageProblem {
    fn enhance(self: &mut Self, times: i32) -> HashSet<(i32, i32)> {
        let mut new_points = HashSet::new();
        for i in 0..times {
            new_points = HashSet::new();
            // Calculate image borders
            let mut min_x = i32::MAX;
            let mut min_y = i32::MAX;
            let mut max_x = i32::MIN;
            let mut max_y = i32::MIN;
            for point in self.points.iter() {
                if point.0 > max_x {
                    max_x = point.0;
                } else if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 > max_y {
                    max_y = point.1;
                } else if point.1 < min_y {
                    min_y = point.1;
                }
            }
            // Flip image
            for y in min_y - 1..max_y + 2 {
                for x in min_x - 1..max_x + 2 {
                    let neighbours = get_neighbours(&x, &y);
                    let bitstring: String = neighbours
                        .iter()
                        .map(|nb| {
                            if min_x <= nb.0 && nb.0 <= max_x && min_y <= nb.1 && nb.1 <= max_y {
                                if self.points.contains(&nb) {
                                    "1"
                                } else {
                                    "0"
                                }
                            } else {
                                if i % 2 == 1 && self.key.chars().nth(0).unwrap() == '#' {
                                    "1"
                                } else {
                                    "0"
                                }
                            }
                        })
                        .collect();
                    let index = bitstring_to_int(&bitstring);
                    if self.key.chars().nth(index as usize).unwrap() == '#' {
                        new_points.insert((x, y));
                    }
                }
            }
            self.points = new_points.clone();
        }
        new_points
    }
}

fn bitstring_to_int(bitstring: &str) -> i64 {
    let mut result = 0;
    let base: i64 = 2;
    for (pos, bit) in bitstring.chars().into_iter().enumerate() {
        result += bit.to_digit(10).unwrap() as i64
            * base.pow((bitstring.len() - 1 - pos).try_into().unwrap());
    }
    result
}

fn get_neighbours(x: &i32, y: &i32) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = vec![];
    let deltas: Vec<i32> = vec![-1, 0, 1];
    for yd in &deltas {
        for xd in &deltas {
            neighbours.push((*x as i32 + xd, *y as i32 + yd));
        }
    }
    neighbours
}

fn read_input_from_file(path: &str) -> ImageProblem {
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut key = String::new();
    let _ = buf.read_line(&mut key);
    let mut points = HashSet::new();
    for (y, line) in buf.lines().enumerate() {
        let line = line.unwrap();
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '#' {
                points.insert((x as i32, (y - 1) as i32));
            }
        }
    }
    ImageProblem {
        key: String::from(key.trim()),
        points,
    }
}

fn main() {
    let mut input = read_input_from_file("input.txt");
    let mut input2 = read_input_from_file("input.txt");
    let part1 = input.enhance(2).len();
    println!("Solution to part 1: {}", &part1);
    let part2 = input2.enhance(50).len();
    println!("Solution to part 2: {}", &part2);
}
