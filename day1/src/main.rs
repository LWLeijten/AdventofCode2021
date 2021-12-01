use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let number = line.trim().parse().unwrap();
            input.push(number);
        });
    input
}

fn calculate_increases_with_chunk_size(input: &Vec<i32>, chunksize: usize) -> i32 {
    let mut previous: Option<i32> = None;
    let mut increases = 0;
    for i in input
        .windows(chunksize)
        .map(|chunk| chunk.iter().sum::<i32>())
    {
        match previous {
            Some(prev) if i > prev => {
                increases += 1;
            }
            _ => {}
        }
        previous = Some(i);
    }
    increases
}

fn main() {
    let input: Vec<i32> = read_input_from_file("input.txt");
    let result_part1 = calculate_increases_with_chunk_size(&input, 1);
    println!("Number of found increases in part 1 is {}", result_part1);
    let result_part2 = calculate_increases_with_chunk_size(&input, 3);
    println!("Number of found increases in part 2 is {}", result_part2);
}
