use std::fs::File;
use std::io::{BufRead, BufReader};

fn simulate_day(fishes: Vec<u64>) -> Vec<u64> {
    let mut new_fishes = vec![0; 9];
    for i in 1..9 {
        new_fishes[i - 1] = fishes[i];
    }
    new_fishes[6] += fishes[0]; // Fishes that went through a birth-cycle.
    new_fishes[8] += fishes[0]; // Newborn fishes
    new_fishes
}

fn read_input_from_file(path: &str) -> Vec<u64> {
    let mut input: Vec<u64> = vec![0; 9];
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);
    let angler_fish: Vec<&str> = line.split(',').collect();
    for af in angler_fish {
        input[af.parse::<usize>().expect("failed to parse number")] += 1;
    }
    input
}

fn calculate_fish_count(input: &Vec<u64>, days: u32) -> u64 {
    let mut fishes = input.clone();
    for _ in 0..days {
        fishes = simulate_day(fishes);
    }
    fishes.into_iter().sum()
}

fn main() {
    let input = read_input_from_file("input.txt");
    let fishes_part1 = calculate_fish_count(&input, 80);
    println!("Solution part 1: {}", fishes_part1);
    let fishes_part2 = calculate_fish_count(&input, 256);
    println!("Solution part 2: {}", fishes_part2);
}
