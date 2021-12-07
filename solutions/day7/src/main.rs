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

fn fuel_costs_a(x: &u32, destination: &u32) -> u32 {
    if x > destination {
        x - destination
    } else {
        destination - x
    }
}

fn fuel_costs_b(x: &u32, destination: &u32) -> u32 {
    if x > destination {
        (1..(x - destination + 1)).fold(0, |a, b| a + b)
    } else {
        (1..(destination - x + 1)).fold(0, |a, b| a + b)
    }
}

fn solve(input: &Vec<u32>, destination: &u32, fuel_costs: fn(&u32, &u32) -> u32) -> u32 {
    input.iter().map(|x| fuel_costs(x, destination)).sum()
}

fn main() {
    let input = read_input_from_file("input.txt");
    println!(
        "Solution to part 1: {}",
        solve(&input, &calculate_median(&input), fuel_costs_a)
    );
    println!(
        "Solution to part 2: {}",
        solve(&input, &calculate_average(&input), fuel_costs_b)
    );
}
