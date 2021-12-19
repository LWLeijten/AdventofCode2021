struct Grid {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn is_in_bounds(pos: &(i32, i32), grid: &Grid) -> bool {
    (grid.min_x..grid.max_x + 1).contains(&pos.0) && (grid.min_y..grid.max_y + 1).contains(&pos.1)
}

fn step(pos: &mut (i32, i32), velocity: &mut (i32, i32), grid: &Grid) -> bool {
    pos.0 += velocity.0;
    pos.1 += velocity.1;
    if velocity.0 > 0 {
        velocity.0 -= 1
    } else if velocity.0 < 0 {
        velocity.0 += 1;
    }
    velocity.1 -= 1;
    if velocity.0 == 0 && velocity.1 < 0 && pos.1 < grid.min_y {
        return false;
    } else {
        return true;
    }
}

fn calculate_peak_for_trajectory(
    pos: &mut (i32, i32),
    grid: &Grid,
    velocity: &mut (i32, i32),
) -> Option<i32> {
    let mut max_y = 0;
    let mut hit_target = false;
    loop {
        if !step(pos, velocity, &grid) {
            break;
        }
        if pos.1 > max_y {
            max_y = pos.1;
        }
        if is_in_bounds(pos, grid) {
            hit_target = true;
        }
    }
    if !hit_target {
        return None;
    } else {
        return Some(max_y);
    }
}

fn part1(grid: &Grid) -> i32 {
    let mut best_y = 0;
    for x in 0..grid.max_x + 1 {
        for y in grid.min_y..-grid.min_y {
            let mut probe = (0, 0);
            let peak = calculate_peak_for_trajectory(&mut probe, &grid, &mut (x, y));
            match peak {
                Some(y_peak) => {
                    if y_peak > best_y {
                        best_y = y_peak
                    }
                }
                None => {}
            }
        }
    }
    best_y
}

fn part2(grid: &Grid) -> i32 {
    let mut hits = 0;
    for x in 0..grid.max_x + 1 {
        for y in grid.min_y..-grid.min_y {
            let mut probe = (0, 0);
            let peak = calculate_peak_for_trajectory(&mut probe, &grid, &mut (x, y));
            match peak {
                Some(_) => {
                    hits += 1;
                }
                None => {}
            }
        }
    }
    hits
}

fn main() {
    let grid = Grid {
        min_x: 57,
        max_x: 116,
        min_y: -198,
        max_y: -148,
    };
    let part1 = part1(&grid);
    println!("Solution to part 1: {}", part1);
    let part2 = part2(&grid);
    println!("Solution to part 2: {}", part2);
}
