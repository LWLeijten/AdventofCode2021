use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(PartialEq, Eq)]
struct Node {
    x: i32,
    y: i32,
    f: i32,
    g: i32,
    h: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn wrap_number(n: &i32, i: i32) -> i32 {
    if *n + i > 9 {
        ((n + i) % 10) + 1
    } else {
        *n + i
    }
}

fn expand_grid(grid: &Vec<Vec<i32>>, times: i32) -> Vec<Vec<i32>> {
    let mut new_grid = grid.clone();
    // Horizontally
    for r in 0..grid.len() {
        for i in 1..times + 1 {
            let mut extension = grid[r].clone();
            extension = extension.iter().map(|n| wrap_number(n, i)).collect();
            new_grid[r].extend(extension);
        }
    }
    // Vertically
    let start_len = new_grid.len();
    for i in 1..times + 1 {
        for r in 0..start_len {
            let mut new_row = new_grid[r].clone();
            new_row = new_row.iter().map(|n| wrap_number(n, i)).collect();
            new_grid.push(new_row);
        }
    }
    new_grid
}

fn within_bounds(grid: &Vec<Vec<i32>>, x: &i32, y: &i32) -> bool {
    *x >= 0 && *x < grid[0].len() as i32 && *y >= 0 && *y < grid.len() as i32
}

fn get_neighbours(heightmap: &Vec<Vec<i32>>, x: &i32, y: &i32) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = vec![];
    for (xx, yy) in [(x, y - 1), (x, y + 1), (&(x - 1), *y), (&(x + 1), *y)] {
        if within_bounds(heightmap, &xx, &yy) {
            neighbours.push((*xx, yy));
        }
    }
    neighbours
}

fn heuristic(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> i32 {
    max(x1, x2) - min(x1, x2) + max(y1, y2) - min(y1, y2)
}

fn a_star(grid: &Vec<Vec<i32>>, destination: (i32, i32)) -> i32 {
    let start = Node {
        x: 0,
        y: 0,
        f: 0,
        g: 0,
        h: 0,
    };
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut open = BinaryHeap::new();
    open.push(start);
    let mut closed = HashSet::new();
    loop {
        let cur = open.pop().unwrap();
        closed.insert((cur.x, cur.y));
        if (cur.x, cur.y) == destination {
            return cur.f;
        }
        let neighbours = get_neighbours(grid, &cur.x, &cur.y);
        for nb in neighbours {
            if closed.contains(&(nb.0, nb.1)) {
                continue;
            }
            let g = cur.g + grid[nb.1 as usize][nb.0 as usize];
            for n in open.iter() {
                if n.x == nb.0 && n.y == nb.1 && g > n.g {
                    continue;
                }
            }
            let h = heuristic(&nb.0, &nb.1, &destination.0, &destination.1);
            let f = g + h;
            let node = Node {
                x: nb.0,
                y: nb.1,
                f,
                g,
                h,
            };
            *came_from.entry((node.x, node.y)).or_insert((0, 0)) = (cur.x, cur.y);
            open.push(node);
        }
    }
}

fn main() {
    let grid = read_input_from_file("input.txt");
    let destination1 = ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32);

    let start = Instant::now();
    let part1 = a_star(&grid, destination1);
    let duration1 = start.elapsed();
    println!("Part 1 solution: {}. Calculated in {:?}", part1, duration1);
    // Part 1 solution: 717. Calculated in 16.304ms

    let new_grid = expand_grid(&grid, 4);
    let destination2 = ((new_grid[0].len() - 1) as i32, (new_grid.len() - 1) as i32);
    let start = Instant::now();
    let part2 = a_star(&new_grid, destination2);
    let duration2 = start.elapsed();
    println!("Part 2 solution: {}. Calculated in {:?}", part2, duration2);
    // Part 2 solution: 2993. Calculated in 491.6688ms
}
