use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct FoldProblem {
    dots: HashSet<(i32, i32)>,
    folds: VecDeque<Fold>,
}

#[derive(Debug)]
struct Fold {
    direction: char,
    index: i32,
}

impl FoldProblem {
    fn apply_next_fold(mut self: Self) -> Self {
        let mut new_dots: HashSet<(i32, i32)> = HashSet::new();
        let fold = self.folds.pop_front().unwrap();
        if fold.direction == 'x' {
            self.dots.iter().for_each(|dot| {
                if dot.0 < fold.index {
                    new_dots.insert(*dot);
                } else {
                    new_dots.insert((fold.index - (dot.0 - fold.index), dot.1));
                }
            });
        } else {
            self.dots.iter().for_each(|dot| {
                if dot.1 < fold.index {
                    new_dots.insert(*dot);
                } else {
                    new_dots.insert((dot.0, fold.index - (dot.1 - fold.index)));
                }
            });
        }
        FoldProblem {
            dots: new_dots,
            folds: self.folds,
        }
    }
}

fn read_input_from_file(path: &str) -> FoldProblem {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    let mut folds = VecDeque::new();
    let mut reading_dots = true;
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            if line.is_empty() {
                reading_dots = false;
            }
            if reading_dots {
                let coordinates: Vec<&str> = line.split(',').collect();
                dots.insert((
                    coordinates[0].parse().unwrap(),
                    coordinates[1].parse().unwrap(),
                ));
            } else if !line.is_empty() {
                let direction: char = line.split_ascii_whitespace().collect::<Vec<&str>>()[2]
                    .chars()
                    .nth(0)
                    .unwrap();
                let index = line.split_ascii_whitespace().collect::<Vec<&str>>()[2]
                    .split('=')
                    .collect::<Vec<&str>>()[1];
                folds.push_back(Fold {
                    direction,
                    index: index.parse().unwrap(),
                });
            }
        });
    FoldProblem { dots, folds }
}

fn part1(mut input: FoldProblem) -> usize {
    input = input.apply_next_fold();
    input.dots.len()
}

fn part2(mut input: FoldProblem) -> FoldProblem {
    loop {
        if input.folds.len() == 0 {
            break;
        }
        input = input.apply_next_fold();
    }
    input
}

fn main() {
    let part1 = part1(read_input_from_file("input.txt"));
    println!("Solution part1: {}", part1);
    println!("Part2");
    let part2 = part2(read_input_from_file("input.txt"));
    let min_x = part2.dots.iter().map(|d| d.0).min().unwrap();
    let max_x = part2.dots.iter().map(|d| d.0).max().unwrap();
    let min_y = part2.dots.iter().map(|d| d.1).min().unwrap();
    let max_y = part2.dots.iter().map(|d| d.1).max().unwrap();
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if part2.dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("{}", '\n');
    }
}
