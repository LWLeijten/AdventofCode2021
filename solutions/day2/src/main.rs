use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Down,
    Up,
}

struct PositionPart1 {
    horizontal_position: i32,
    depth: i32,
}

struct PositionPart2 {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

struct Command {
    direction: Direction,
    units: i32,
}

impl Command {
    fn new(direction_string: &str, units: i32) -> Self {
        let direction = match direction_string {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::Up,
        };
        Command { direction, units }
    }
}

fn read_input_from_file(path: &str) -> Vec<Command> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let line_vec: Vec<&str> = line.trim().split(" ").collect();
            input.push(Command::new(&line_vec[0], line_vec[1].parse().unwrap()));
        });
    input
}

fn calculate_final_position_part1(commands: &Vec<Command>) -> PositionPart1 {
    let mut pos = PositionPart1 {
        horizontal_position: 0,
        depth: 0,
    };
    for c in commands {
        match c.direction {
            Direction::Forward => pos.horizontal_position += c.units,
            Direction::Down => pos.depth += c.units,
            Direction::Up => pos.depth -= c.units,
        }
    }
    pos
}

fn calculate_final_position_part2(commands: &Vec<Command>) -> PositionPart1 {
    let mut pos = PositionPart2 {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };
    for c in commands {
        match c.direction {
            Direction::Forward => {
                pos.horizontal_position += c.units;
                pos.depth += c.units * pos.aim
            }
            Direction::Down => pos.aim += c.units,
            Direction::Up => pos.aim -= c.units,
        }
    }
    PositionPart1 {
        horizontal_position: pos.horizontal_position,
        depth: pos.depth,
    }
}

fn print_solution(part: i8, pos: PositionPart1) {
    println!(
        "Part{}: Horizontal position: {}, depth: {}, multiplication: {}",
        part,
        pos.horizontal_position,
        pos.depth,
        pos.horizontal_position * pos.depth
    );
}

fn main() {
    let input: Vec<Command> = read_input_from_file("input.txt");
    let result_part1 = calculate_final_position_part1(&input);
    print_solution(1, result_part1);
    let result_part2 = calculate_final_position_part2(&input);
    print_solution(2, result_part2);
}
