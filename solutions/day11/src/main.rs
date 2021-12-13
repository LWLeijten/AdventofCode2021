use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            let num_vec = line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();
            input.push(num_vec);
        });
    input
}

fn within_bounds(grid: &Vec<Vec<i32>>, x: &i32, y: &i32) -> bool {
    *x >= 0 && *x < grid[0].len() as i32 && *y >= 0 && *y < grid.len() as i32
}

fn get_neighbours(grid: &Vec<Vec<i32>>, x: &usize, y: &usize) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = vec![];
    let deltas: Vec<i32> = vec![-1, 0, 1];
    for yd in &deltas {
        for xd in &deltas {
            if !(*yd == 0 && *xd == 0) {
                if within_bounds(grid, &(*x as i32 + *xd), &(*y as i32 + *yd)) {
                    neighbours.push((*x as i32 + xd, *y as i32 + yd));
                }
            }
        }
    }
    neighbours
}

fn increase_energy_levels(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..grid.len() as i32 {
        grid[i as usize] = grid[i as usize].iter().map(|n| n + 1).collect();
    }
    grid
}

fn flash_octopi(mut grid: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let mut has_flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut to_flash = VecDeque::new();
    let mut flashes = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] > 9 {
                to_flash.push_back((x, y));
            }
        }
    }
    loop {
        if to_flash.len() == 0 {
            break;
        }
        let cur = to_flash.pop_front().unwrap();
        has_flashed.insert(cur);
        flashes += 1;
        let neighbours = get_neighbours(&grid, &cur.0, &cur.1);
        for nb in neighbours {
            grid[nb.1 as usize][nb.0 as usize] += 1;
            if grid[nb.1 as usize][nb.0 as usize] > 9
                && !has_flashed.contains(&(nb.0 as usize, nb.1 as usize))
                && !to_flash.contains(&(nb.0 as usize, nb.1 as usize))
            {
                to_flash.push_back((nb.0 as usize, nb.1 as usize));
            }
        }
    }
    (grid, flashes)
}

fn reset_energy_levels(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..grid.len() {
        grid[i] = grid[i]
            .iter()
            .map(|n| if *n > 9 { 0 } else { *n })
            .collect();
    }
    grid
}

fn part1(mut grid: Vec<Vec<i32>>, steps: u32) -> i32 {
    let mut flashes = 0;
    for _ in 0..steps {
        grid = increase_energy_levels(grid);
        let res_tup = flash_octopi(grid);
        grid = res_tup.0;
        flashes += res_tup.1;
        grid = reset_energy_levels(grid);
    }
    flashes
}

fn part2(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut loops = 0;
    loop {
        if grid.iter().all(|row| row.into_iter().all(|num| *num == 0)) {
            break;
        }
        grid = increase_energy_levels(grid);
        let res_tup = flash_octopi(grid);
        grid = res_tup.0;
        grid = reset_energy_levels(grid);
        loops += 1;
    }
    loops
}

fn main() {
    let input = read_input_from_file("input.txt");
    let input2 = input.clone();
    let part1 = part1(input, 100);
    println!("Solution to part 1: {}", &part1);
    let part2 = part2(input2);
    println!("Solution to part 2: {}", &part2);
}
