use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

fn read_input_from_file(path: &str) -> Vec<Entry> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut entries: Vec<Entry> = vec![];
    for line in buf.lines() {
        let unwrapped = line.unwrap();
        let split_line: Vec<_> = unwrapped.split('|').collect();
        let patterns: Vec<String> = split_line[0]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let outputs: Vec<String> = split_line[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        entries.push(Entry { patterns, outputs });
    }
    entries
}

fn calculate_common_characters(string1: &String, string2: &String) -> u32 {
    let mut commons = 0;
    for char in string1.chars() {
        if string2.contains(char) {
            commons += 1;
        }
    }
    commons
}

fn remove_char_from_pattern(pattern: &String, to_remove: char) -> String {
    let mut pattern_clone = pattern.clone();
    pattern_clone = pattern_clone.replace(to_remove, "");
    pattern_clone
}

fn patterns_are_equal(pattern1: &String, pattern2: &String) -> bool {
    calculate_common_characters(pattern1, pattern2) == pattern1.len() as u32
        && pattern1.len() == pattern2.len()
}

fn calculate_output(digit1: &u32, digit2: &u32, digit3: &u32, digit4: &u32) -> u32 {
    1000 * digit1 + 100 * digit2 + 10 * digit3 + digit4
}

fn find_zero<'a>(entry: &'a Entry, five_pattern: &'a String) -> &'a String {
    let mut candidates = entry
        .patterns
        .iter()
        .filter(|p| p.len() == 6)
        .collect::<Vec<&String>>();
    candidates.retain(|c| calculate_common_characters(c, five_pattern) == 4);
    candidates[0]
}

fn find_one(entry: &Entry) -> &String {
    entry
        .patterns
        .iter()
        .filter(|p| p.len() == 2)
        .collect::<Vec<&String>>()[0]
}

fn find_two<'a>(
    entry: &'a Entry,
    three_pattern: &'a String,
    five_pattern: &'a String,
) -> &'a String {
    let mut candidates = entry
        .patterns
        .iter()
        .filter(|p| p.len() == 5)
        .collect::<Vec<&String>>();
    candidates.retain(|c| {
        calculate_common_characters(c, three_pattern) < 5
            && calculate_common_characters(c, five_pattern) < 5
    });
    candidates[0]
}

fn find_three<'a>(entry: &'a Entry, one_pattern: &'a String) -> &'a String {
    let mut candidates = entry
        .patterns
        .iter()
        .filter(|p| p.len() == 5)
        .collect::<Vec<&String>>();
    candidates.retain(|c| calculate_common_characters(c, one_pattern) == 2);
    candidates[0]
}

fn find_four(entry: &Entry) -> &String {
    entry
        .patterns
        .iter()
        .filter(|p| p.len() == 4)
        .collect::<Vec<&String>>()[0]
}

fn find_five<'a>(
    entry: &'a Entry,
    nine_pattern: &'a String,
    one_pattern: &'a String,
) -> &'a String {
    let candidates = entry
        .patterns
        .iter()
        .filter(|p| p.len() == 5)
        .collect::<Vec<&String>>();
    let possibility_a =
        remove_char_from_pattern(nine_pattern, one_pattern.chars().collect::<Vec<char>>()[0]);
    let possibility_b =
        remove_char_from_pattern(nine_pattern, one_pattern.chars().collect::<Vec<char>>()[1]);
    let candidates_a = &candidates
        .iter()
        .filter(|p| calculate_common_characters(p, &possibility_a) == 5)
        .collect::<Vec<&&String>>();
    let candidates_b = &candidates
        .iter()
        .filter(|p| calculate_common_characters(p, &possibility_b) == 5)
        .collect::<Vec<&&String>>();
    if candidates_a.into_iter().len() > 0 {
        return candidates_a[0];
    } else {
        return candidates_b[0];
    }
}

fn find_six<'a>(
    entry: &'a Entry,
    five_pattern: &'a String,
    nine_pattern: &'a String,
) -> &'a String {
    let mut candidates = entry
        .patterns
        .iter()
        .filter(|p| p.len() == 6)
        .collect::<Vec<&String>>();
    candidates.retain(|c| {
        calculate_common_characters(c, five_pattern) == 5
            && calculate_common_characters(c, nine_pattern) == 5
    });
    candidates[0]
}

fn find_seven(entry: &Entry) -> &String {
    entry
        .patterns
        .iter()
        .filter(|p| p.len() == 3)
        .collect::<Vec<&String>>()[0]
}

fn find_eight(entry: &Entry) -> &String {
    entry
        .patterns
        .iter()
        .filter(|p| p.len() == 7)
        .collect::<Vec<&String>>()[0]
}

fn find_nine<'a>(entry: &'a Entry, four_pattern: &'a String) -> &'a String {
    let mut candidates = entry
        .patterns
        .iter()
        .filter(|p| p.len() == 6)
        .collect::<Vec<&String>>();
    candidates.retain(|c| calculate_common_characters(c, four_pattern) == 4);
    candidates[0]
}

fn find_digit_by_pattern<'a>(
    mapping: &'a HashMap<u32, &'a String>,
    pattern: &'a String,
) -> &'a u32 {
    mapping
        .iter()
        .find_map(|(k, v)| {
            if patterns_are_equal(&**v, &pattern) {
                Some(k)
            } else {
                None
            }
        })
        .unwrap()
}

fn solve_entry(entry: &Entry) -> u32 {
    let mut mapping: HashMap<u32, &String> = HashMap::new();
    mapping.insert(1, find_one(entry));
    mapping.insert(4, find_four(entry));
    mapping.insert(7, find_seven(entry));
    mapping.insert(8, find_eight(entry));
    mapping.insert(3, find_three(entry, mapping.get(&1).unwrap()));
    mapping.insert(9, find_nine(entry, mapping.get(&4).unwrap()));
    mapping.insert(
        5,
        find_five(entry, mapping.get(&9).unwrap(), mapping.get(&1).unwrap()),
    );
    mapping.insert(
        2,
        find_two(entry, mapping.get(&3).unwrap(), mapping.get(&5).unwrap()),
    );
    mapping.insert(0, find_zero(entry, mapping.get(&5).unwrap()));
    mapping.insert(
        6,
        find_six(entry, mapping.get(&5).unwrap(), mapping.get(&9).unwrap()),
    );
    let digit1 = find_digit_by_pattern(&mapping, &entry.outputs[0]);
    let digit2 = find_digit_by_pattern(&mapping, &entry.outputs[1]);
    let digit3 = find_digit_by_pattern(&mapping, &entry.outputs[2]);
    let digit4 = find_digit_by_pattern(&mapping, &entry.outputs[3]);
    calculate_output(digit1, digit2, digit3, digit4)
}

fn part1(input: &Vec<Entry>) -> u32 {
    let mut digits = 0;
    for entry in input {
        for output in &entry.outputs {
            if vec![2, 4, 3, 7].contains(&output.len()) {
                digits += 1;
            }
        }
    }
    digits
}

fn part2(input: &Vec<Entry>) -> u32 {
    let mut solution = 0;
    for entry in input {
        solution += solve_entry(entry);
    }
    solution
}

fn main() {
    let input = read_input_from_file("input.txt");
    let part1 = part1(&input);
    println!("Solution to part 1: {}", part1);
    let part2 = part2(&input);
    println!("Solution to part 2: {}", part2);
}
