use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let char_vec = line.trim().chars().collect();
            input.push(char_vec);
        });
    input
}

fn bitstring_to_int(bitstring: &Vec<i64>) -> i64 {
    let mut result = 0;
    let base: i64 = 2;
    for (pos, bit) in bitstring.into_iter().enumerate() {
        result += bit * base.pow((bitstring.len() - 1 - pos).try_into().unwrap());
    }
    result
}

fn calculate_gamma(input: &Vec<Vec<char>>) -> Vec<i64> {
    let length = input[0].len();
    let mut zeroes: Vec<i64> = vec![0; length];
    let mut ones: Vec<i64> = vec![0; length];
    for bits in input {
        for (pos, bit) in bits.iter().enumerate() {
            match bit {
                '0' => zeroes[pos] += 1,
                '1' => ones[pos] += 1,
                _ => panic!(),
            }
        }
    }
    let mut gamma: Vec<i64> = vec![0; length];
    for i in 0..length {
        gamma[i] = if zeroes[i] > ones[i] { 0 } else { 1 };
    }
    gamma
}

fn calculate_epsilon(gamma: &Vec<i64>) -> Vec<i64> {
    let epsilon: Vec<i64> = gamma
        .clone()
        .into_iter()
        .map(|x| if x == 1 { 0 } else { 1 })
        .collect();
    epsilon
}

fn calculate_oxygen(input: &Vec<Vec<char>>) -> Vec<i64> {
    let mut input_copy = input.clone();
    for i in 0..input[0].len() {
        let mut hashmap: HashMap<char, i64> = HashMap::from([('0', 0), ('1', 0)]);
        for bits in &input_copy {
            *hashmap.get_mut(&bits[i]).unwrap() += 1;
        }
        if hashmap[&'0'] > hashmap[&'1'] {
            input_copy.retain(|bits| bits[i] == '0');
        } else {
            input_copy.retain(|bits| bits[i] == '1');
        }
        if input_copy.len() == 1 as usize {
            break;
        }
    }
    input_copy[0]
        .iter()
        .map(|x| if *x == '0' { 0 } else { 1 })
        .collect()
}

fn calculate_carbonmonoxide(input: &Vec<Vec<char>>) -> Vec<i64> {
    let mut input_copy = input.clone();
    for i in 0..input[0].len() {
        let mut hashmap: HashMap<char, i64> = HashMap::from([('0', 0), ('1', 0)]);
        for bits in &input_copy {
            *hashmap.get_mut(&bits[i]).unwrap() += 1;
        }
        if hashmap[&'0'] <= hashmap[&'1'] {
            input_copy.retain(|bits| bits[i] == '0');
        } else {
            input_copy.retain(|bits| bits[i] == '1');
        }
        if input_copy.len() == 1 as usize {
            break;
        }
    }
    input_copy[0]
        .iter()
        .map(|x| if *x == '0' { 0 } else { 1 })
        .collect()
}

fn part1(input: &Vec<Vec<char>>) -> i64 {
    let gamma_binary = calculate_gamma(&input);
    let epsilon_binary = calculate_epsilon(&gamma_binary);
    bitstring_to_int(&gamma_binary) * bitstring_to_int(&epsilon_binary)
}

fn part2(input: &Vec<Vec<char>>) -> i64 {
    let oxygen_binary = calculate_oxygen(&input);
    let carbonmonoxide_binary = calculate_carbonmonoxide(&input);
    bitstring_to_int(&oxygen_binary) * bitstring_to_int(&carbonmonoxide_binary)
}

fn main() {
    let input = read_input_from_file("input.txt");
    let result_part1 = part1(&input);
    let result_part2 = part2(&input);
    println!("Result for part 1: {}", result_part1);
    println!("Result for part 1: {}", result_part2);
}
