use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_horizontal(self: &Self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(self: &Self) -> bool {
        self.p1.x == self.p2.x
    }

    fn all_points_on_line(self: &Self, include_diagonals: bool) -> Vec<Point> {
        let mut points = vec![];
        let (left_point, righ_point) = if self.p1.x < self.p2.x {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
        };
        if self.is_horizontal() {
            for x in left_point.x..righ_point.x + 1 {
                points.push(Point { x, y: self.p1.y });
            }
        } else if self.is_vertical() {
            for y in min(self.p1.y, self.p2.y)..max(self.p1.y, self.p2.y) + 1 {
                points.push(Point { x: self.p1.x, y });
            }
        } else if include_diagonals {
            let mut cur_y = left_point.y;
            for x in left_point.x..righ_point.x + 1 {
                points.push(Point { x, y: cur_y });
                if cur_y < righ_point.y {
                    cur_y += 1
                } else if cur_y > 0 {
                    cur_y -= 1
                }
            }
        }
        points
    }
}

fn read_input_from_file(path: &str) -> Vec<Line> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut lines = vec![];
    buf.lines().for_each(|line| {
        let cur_line = line.unwrap();
        let points: Vec<&str> = cur_line.split("->").collect();
        let p1: Vec<&str> = points[0].split(',').collect();
        let p2: Vec<&str> = points[1].split(',').collect();
        let line = Line {
            p1: Point {
                x: p1[0]
                    .trim()
                    .parse()
                    .expect("Failed to parse coordinate X for p1"),
                y: p1[1]
                    .trim()
                    .parse()
                    .expect("Failed to parse coordinate Y for p1"),
            },
            p2: Point {
                x: p2[0]
                    .trim()
                    .parse()
                    .expect("Failed to parse coordinate X for p2"),
                y: p2[1]
                    .trim()
                    .parse()
                    .expect("Failed to parse coordinate Y for p2"),
            },
        };
        lines.push(line);
    });
    lines
}

fn calculate_solution(lines: &Vec<Line>, include_digonals: bool) -> usize {
    let mut vents: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        let points = line.all_points_on_line(include_digonals);
        for point in points {
            *vents.entry(point).or_insert(0) += 1;
        }
    }
    vents.retain(|_, count| count > &mut 1);
    vents.len()
}

fn main() {
    let lines = read_input_from_file("input.txt");
    let part1 = calculate_solution(&lines, false);
    println!("Solution for part1: {}", part1);
    let part2 = calculate_solution(&lines, true);
    println!("Solution for part2: {}", part2);
}
