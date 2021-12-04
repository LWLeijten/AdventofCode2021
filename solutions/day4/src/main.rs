use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

struct BingoProblem {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

#[derive(Clone)]
struct BingoBoard {
    board: Vec<Vec<u32>>,
}

impl BingoProblem {
    fn find_nth_winner(self: &Self, n: usize) -> Option<(BingoBoard, Vec<u32>)> {
        let mut winners: HashSet<usize> = HashSet::new();
        for i in 0..self.numbers.len() {
            let vec_slice = self.numbers[0..i].to_vec();
            for (index, bb) in self.boards.iter().enumerate() {
                if !winners.contains(&index) && bb.check_if_winning(&vec_slice) {
                    if winners.len() == n - 1 {
                        return Some((bb.clone(), vec_slice));
                    } else {
                        winners.insert(index);
                    }
                }
            }
        }
        None
    }
}

impl BingoBoard {
    fn calculate_score(self: &Self, called_nums: &Vec<u32>) -> u32 {
        let mut flattened_board = self
            .board
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<u32>>();
        flattened_board.retain(|x| !called_nums.contains(x));
        let res: u32 = flattened_board.iter().sum();
        res.clone() * called_nums.last().expect("Lastnum not found")
    }

    fn check_if_winning(self: &Self, numbers: &Vec<u32>) -> bool {
        for row in &self.board {
            if row.iter().all(|x| numbers.contains(x)) {
                return true;
            }
        }
        for col in 0..self.board[0].len() {
            if self.board.iter().all(|vec| numbers.contains(&vec[col])) {
                return true;
            }
        }
        false
    }
}

fn read_input_from_file(path: &str) -> BingoProblem {
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    let mut numberstring = String::new();
    let _ = buf.read_line(&mut numberstring);
    let numbers: Vec<u32> = numberstring
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().expect("parse error"))
        .collect();
    let mut boards: Vec<BingoBoard> = vec![];
    let mut curboard: BingoBoard = BingoBoard { board: vec![] };
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            match line.as_str() {
                "" => {
                    if curboard.board.len() > 0 {
                        boards.push(curboard.clone());
                    }
                    curboard = BingoBoard { board: vec![] };
                }
                _ => {
                    curboard.board.push(
                        line.trim()
                            .split_whitespace()
                            .map(|x| x.parse::<u32>().expect("parse error"))
                            .collect(),
                    );
                }
            }
        });
    boards.push(curboard.clone());
    BingoProblem { numbers, boards }
}

fn part1(input: &BingoProblem) -> u32 {
    let (board, numbers) = input.find_nth_winner(1).expect("Winning board not found.");
    let score = board.calculate_score(&numbers);
    score
}

fn part2(input: &BingoProblem) -> u32 {
    let (board, numbers) = input
        .find_nth_winner(input.boards.len())
        .expect("Worst board not found");
    let score = board.calculate_score(&numbers);
    score
}

fn main() {
    let input = read_input_from_file("input.txt");
    let part1 = part1(&input);
    println!("Solution part1: {}", part1);
    let part2 = part2(&input);
    println!("Solution part2: {}", part2);
}
