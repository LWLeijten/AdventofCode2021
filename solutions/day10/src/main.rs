use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            input.push(line);
        });
    input
}

fn closes_previous(previous: &char, current: &char) -> bool {
    let mut matches: bool = false;
    match previous {
        '(' => {
            matches = current == &')';
        }
        '[' => {
            matches = current == &']';
        }
        '{' => {
            matches = current == &'}';
        }
        '<' => {
            matches = current == &'>';
        }
        _ => {}
    }
    matches
}

fn get_error_score(c: &char) -> u32 {
    match c {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _ => return 0,
    }
}

fn get_completion_score(c: &char) -> u64 {
    match c {
        '(' => return 1,
        '[' => return 2,
        '{' => return 3,
        '<' => return 4,
        _ => return 0,
    }
}

fn calc_error_score(line: &String) -> u32 {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if ['(', '[', '{', '<'].contains(&c) {
            stack.push(c);
        } else {
            let try_prev = stack.pop();
            match try_prev {
                Some(prev) => {
                    if !closes_previous(&prev, &c) {
                        return get_error_score(&c);
                    }
                }
                None => {}
            }
        }
    }
    0
}

fn calc_completion_score(line: &String) -> u64 {
    let mut score = 0;
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if ['(', '[', '{', '<'].contains(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    for i in (0..stack.len()).rev() {
        score = score * 5 + get_completion_score(&stack[i]);
    }
    score
}

fn part1(input: &Vec<String>) -> u32 {
    input.iter().map(|l| calc_error_score(l)).sum()
}

fn part2(mut input: Vec<String>) -> u64 {
    input.retain(|l| calc_error_score(l) == 0);
    let mut complete_scores: Vec<u64> = input.iter().map(|l| calc_completion_score(l)).collect();
    complete_scores.sort();
    complete_scores[(complete_scores.len() - 1) / 2]
}

fn main() {
    let input = read_input_from_file("input.txt");
    let part1 = part1(&input);
    println!("Solution to part1: {}", part1);
    let part2 = part2(input);
    println!("Solution to part2: {}", part2);
}
