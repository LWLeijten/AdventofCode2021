use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn read_input_from_file(path: &str) -> Vec<CubeInstruction> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut instructions = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let on_regex = Regex::new(r"(on)|(off)").unwrap();
            let x_regex = Regex::new(r"x=(?P<range>-?\d+..-?\d+)").unwrap();
            let y_regex = Regex::new(r"y=(?P<range>-?\d+..-?\d+)").unwrap();
            let z_regex = Regex::new(r"z=(?P<range>-?\d+..-?\d+)").unwrap();
            let on = on_regex.find(&line).unwrap().as_str();
            let mut x_range: Vec<i64> = x_regex
                .captures(&line)
                .and_then(|cap| cap.name("range").map(|range| range.as_str().split("..")))
                .unwrap()
                .map(|s| s.parse().unwrap())
                .collect();
            let mut y_range: Vec<i64> = y_regex
                .captures(&line)
                .and_then(|cap| cap.name("range").map(|range| range.as_str().split("..")))
                .unwrap()
                .map(|s| s.parse().unwrap())
                .collect();
            let mut z_range: Vec<i64> = z_regex
                .captures(&line)
                .and_then(|cap| cap.name("range").map(|range| range.as_str().split("..")))
                .unwrap()
                .map(|s| s.parse().unwrap())
                .collect();
            x_range.sort();
            y_range.sort();
            z_range.sort();
            instructions.push(CubeInstruction {
                cube: Cube {
                    x_min: x_range[0],
                    x_max: x_range[1],
                    y_min: y_range[0],
                    y_max: y_range[1],
                    z_min: z_range[0],
                    z_max: z_range[1],
                },
                on: if on == "on" { true } else { false },
            });
        });
    instructions
}

struct CubeInstruction {
    cube: Cube,
    on: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl Cube {
    fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64, z_min: i64, z_max: i64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn get_mass(self: &Self) -> i64 {
        ((self.x_max - self.x_min) + 1)
            * ((self.y_max - self.y_min) + 1)
            * ((self.z_max - self.z_min) + 1)
    }

    fn intersects_with(self: &Self, other: &Cube) -> bool {
        let xs = max(self.x_min, other.x_min) <= min(self.x_max, other.x_max);
        let ys = max(self.y_min, other.y_min) <= min(self.y_max, other.y_max);
        let zs = max(self.z_min, other.z_min) <= min(self.z_max, other.z_max);
        xs && ys && zs
    }

    fn get_intersecting_cubes(self: &Self, cubes: &Vec<Cube>) -> Vec<Cube> {
        let mut intersections = vec![];
        for c in cubes {
            if self.intersects_with(&c) {
                intersections.push(c.clone());
            }
        }
        intersections
    }

    fn split_into_sub_cubes(self: &mut Self, other: &Cube) -> Vec<Cube> {
        let mut sub_cubes = vec![];
        // X-axis
        if other.x_min > self.x_min {
            sub_cubes.push(Cube::new(
                self.x_min,
                other.x_min - 1,
                self.y_min,
                self.y_max,
                self.z_min,
                self.z_max,
            ));
            self.x_min = other.x_min;
        }
        if other.x_max < self.x_max {
            sub_cubes.push(Cube::new(
                other.x_max + 1,
                self.x_max,
                self.y_min,
                self.y_max,
                self.z_min,
                self.z_max,
            ));
            self.x_max = other.x_max;
        }
        // Y-axis
        if other.y_min > self.y_min {
            sub_cubes.push(Cube::new(
                self.x_min,
                self.x_max,
                self.y_min,
                other.y_min - 1,
                self.z_min,
                self.z_max,
            ));
            self.y_min = other.y_min;
        }
        if other.y_max < self.y_max {
            sub_cubes.push(Cube::new(
                self.x_min,
                self.x_max,
                other.y_max + 1,
                self.y_max,
                self.z_min,
                self.z_max,
            ));
            self.y_max = other.y_max;
        }
        // Z-axis
        if other.z_min > self.z_min {
            sub_cubes.push(Cube::new(
                self.x_min,
                self.x_max,
                self.y_min,
                self.y_max,
                self.z_min,
                other.z_min - 1,
            ));
            self.z_min = other.z_min;
        }
        if other.z_max < self.z_max {
            sub_cubes.push(Cube::new(
                self.x_min,
                self.x_max,
                self.y_min,
                self.y_max,
                other.z_max + 1,
                self.z_max,
            ));
            self.z_max = other.z_max;
        }
        sub_cubes
    }
}

fn cube_in_region_1(cube: &Cube) -> bool {
    if cube.x_min < -50 {
        return false;
    } else if cube.x_max > 50 {
        return false;
    } else if cube.y_min < -50 {
        return false;
    } else if cube.y_max > 50 {
        return false;
    } else if cube.z_min < -50 {
        return false;
    } else if cube.z_max > 50 {
        return false;
    }
    true
}

fn solve(instructions: &Vec<CubeInstruction>, part1: bool) -> i64 {
    let mut cubes: Vec<Cube> = vec![];
    for i in instructions {
        if !part1 || cube_in_region_1(&i.cube) {
            let cube = i.cube.clone();
            let mut intersections = cube.get_intersecting_cubes(&cubes);
            for other in intersections.iter_mut() {
                cubes.retain(|cb| cb != other);
                let sub_cubes = other.split_into_sub_cubes(&cube);
                for sc in sub_cubes {
                    cubes.push(sc);
                }
            }
            if i.on {
                cubes.push(cube);
            }
        }
    }
    cubes.iter().map(|c| c.get_mass()).sum()
}

fn main() {
    let instructions = read_input_from_file("input.txt");
    let solution1 = solve(&instructions, true);
    println!("Solution part1: {}", &solution1);
    let solution2 = solve(&instructions, false);
    println!("Solution part2: {}", &solution2);
}
