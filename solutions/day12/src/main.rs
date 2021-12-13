use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut graph = HashMap::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let line_split = line.split('-').collect::<Vec<&str>>();
            graph
                .entry(String::from(line_split[0]))
                .or_insert(vec![])
                .push(String::from(line_split[1]));
            graph
                .entry(String::from(line_split[1]))
                .or_insert(vec![])
                .push(String::from(line_split[0]));
        });
    graph
}

fn is_large_cave(cave: &String) -> bool {
    cave.to_ascii_uppercase() == *cave
}

fn find_all_paths(
    graph: &HashMap<String, Vec<String>>,
    path: Vec<String>,
    small_cave_constraint: i32,
) -> Vec<Vec<String>> {
    let current = path.last().unwrap();
    if current == "end" {
        return vec![path];
    }
    let mut paths: Vec<Vec<String>> = vec![];
    let neighbours = graph.get(current).unwrap();
    for nb in neighbours {
        if nb != "start" {
            if is_large_cave(nb)
                || !path.contains(nb)
                || path.iter().all(|v| {
                    is_large_cave(v)
                        || path.iter().filter(|w| *w == v).count()
                            < small_cave_constraint.try_into().unwrap()
                })
            {
                let mut new_path = path.clone();
                new_path.push(nb.to_string());
                paths.append(&mut find_all_paths(&graph, new_path, small_cave_constraint));
            }
        }
    }
    paths
}

fn main() {
    let graph = read_input_from_file("input.txt");
    let paths1 = find_all_paths(&graph, vec![String::from("start")], 1);
    println!("Solution to part 1: {}", paths1.len());
    let paths2 = find_all_paths(&graph, vec![String::from("start")], 2);
    println!("Solution to part 2: {}", paths2.len());
}
