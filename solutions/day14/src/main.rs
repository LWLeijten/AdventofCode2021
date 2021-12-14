use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PolymerProblem {
    template: String,
    rules: Vec<PairInsertion>,
}

struct PairInsertion {
    antecedent: String,
    consequent: String,
}

fn read_input_from_file(path: &str) -> PolymerProblem {
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut template = String::new();
    let _ = buf.read_line(&mut &mut template);
    let mut rules = vec![];
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            if !line.is_empty() {
                let split: Vec<&str> = line.split("->").collect();
                rules.push(PairInsertion {
                    antecedent: String::from(split[0].trim()),
                    consequent: String::from(split[1].trim()),
                })
            }
        });
    PolymerProblem {
        template: template.trim().to_string(),
        rules,
    }
}

fn run_n_times(template: String, rules: Vec<PairInsertion>, n: u64) -> u64 {
    let mut freq_map: HashMap<String, u64> = HashMap::new();
    let mut element_counts: HashMap<char, u64> = HashMap::new();
    let antecedents: Vec<String> = rules.iter().map(|r| r.antecedent.clone()).collect();
    // Initial character counts
    for c in template.chars() {
        *element_counts.entry(c).or_insert(0) += 1;
    }
    // Insert all known antecedents
    for i in 0..rules.len() {
        freq_map.insert(rules[i].antecedent.clone(), 0);
    }
    // Initial antecedent counts
    for i in 0..template.len() - 1 {
        let chunk = &template[i..i + 2];
        *freq_map.get_mut(chunk).unwrap() += 1;
    }
    // Iterate
    for _ in 0..n {
        let loop_start_freqs = freq_map.clone();
        for a in antecedents.iter() {
            let count = *loop_start_freqs.get(a).unwrap();
            if count > 0 {
                let consequent = rules
                    .iter()
                    .filter(|r| &r.antecedent == a)
                    .map(|r| r.consequent.clone())
                    .collect::<String>()
                    .chars()
                    .nth(0)
                    .unwrap();
                let new_a: String = format!("{}{}", a.chars().nth(0).unwrap(), consequent);
                let new_b: String = format!("{}{}", consequent, a.chars().nth(1).unwrap());
                *freq_map.get_mut(&new_a).unwrap() += count;
                *freq_map.get_mut(&new_b).unwrap() += count;
                *freq_map.get_mut(a).unwrap() -= count;
                *element_counts.entry(consequent).or_insert(0) += count;
            }
        }
    }
    element_counts.iter().max_by_key(|entry| entry.1).unwrap().1
        - element_counts.iter().min_by_key(|entry| entry.1).unwrap().1
}
fn main() {
    let input = read_input_from_file("input.txt");
    // let part1 = run_n_times(input.template, input.rules, 10);
    // println!("Part 1 solution: {}", part1);
    let part2 = run_n_times(input.template, input.rules, 40);
    println!("Part 2 solution: {}", part2);
}
