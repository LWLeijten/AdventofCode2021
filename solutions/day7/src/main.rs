use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> Vec<u32> {
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);
    let mut numbers: Vec<u32> = line
        .split(',')
        .map(|n| n.parse().expect("Number parse error"))
        .collect();
    numbers.sort();
    numbers
}

fn calculate_median(input: &Vec<u32>) -> u32 {
    input[(input.len() + 1) / 2]
}

fn calculate_average(input: &Vec<u32>) -> u32 {
    input.iter().sum::<u32>() / input.len() as u32
}

fn part1(input: &Vec<u32>, median: &u32) -> u32 {
    input
        .iter()
        .map(|x| if x > median { x - median } else { median - x })
        .sum()
}

fn part2(input: &Vec<u32>, average: &u32) -> u32 {
    input
        .iter()
        .map(|x| {
            if x > average {
                (1..(x - average + 1)).fold(0, |a, b| a + b)
            } else {
                (1..(average - x + 1)).fold(0, |a, b| a + b)
            }
        })
        .sum()
}

fn main() {
    let input = read_input_from_file("input.txt");
    println!(
        "Solution to part 1: {}",
        part1(&input, &calculate_median(&input))
    );
    println!(
        "Solution to part 2: {}",
        part2(&input, &calculate_average(&input))
    );
}
