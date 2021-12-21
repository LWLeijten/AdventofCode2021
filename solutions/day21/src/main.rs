use std::{cmp::max, collections::HashMap};

fn part1(positions: &mut Vec<i32>) -> i32 {
    let mut d_die = 1;
    let mut rolls = 0;
    let mut scores = vec![0, 0];
    while scores[0] < 1000 && scores[1] < 1000 {
        let player = if rolls % 6 == 0 { 0 } else { 1 };
        for _ in 0..3 {
            positions[player] = ((positions[player] + d_die - 1) % 10) + 1;
            d_die = d_die % 100 + 1;
        }
        scores[player] += positions[player];
        rolls += 3;
    }
    rolls * scores.iter().min().unwrap()
}

fn part2(
    p1: i64,
    p1_score: i64,
    p2: i64,
    p2_score: i64,
    turn: i32,
    cache: &mut HashMap<(i64, i64, i64, i64, i32), (i64, i64)>,
) -> (i64, i64) {
    let cache_key = (p1, p1_score, p2, p2_score, turn);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    } else if p1_score >= 21 {
        return (1, 0);
    } else if p2_score >= 21 {
        return (0, 1);
    }
    let mut timelines = vec![];
    if turn % 2 == 0 {
        for dice1 in 1..4 {
            for dice2 in 1..4 {
                for dice3 in 1..4 {
                    let rolled = dice1 + dice2 + dice3;
                    let new_p1 = ((p1 + rolled - 1) % 10) + 1;
                    timelines.push(part2(
                        new_p1,
                        p1_score + new_p1,
                        p2,
                        p2_score,
                        turn + 1,
                        cache,
                    ))
                }
            }
        }
    } else {
        for dice1 in 1..4 {
            for dice2 in 1..4 {
                for dice3 in 1..4 {
                    let rolled = dice1 + dice2 + dice3;
                    let new_p2 = ((p2 + rolled - 1) % 10) + 1;
                    timelines.push(part2(
                        p1,
                        p1_score,
                        new_p2,
                        p2_score + new_p2,
                        turn + 1,
                        cache,
                    ))
                }
            }
        }
    }
    let mut wins = (0, 0);
    for result in timelines {
        wins.0 += result.0;
        wins.1 += result.1;
    }
    cache.insert(cache_key, wins);
    wins
}

fn main() {
    let part1 = part1(&mut vec![10, 4]);
    println!("Solution to part 1: {}", &part1);
    let part2 = part2(10, 0, 4, 0, 0, &mut HashMap::new());
    println!("Solution to part 2: {}", max(part2.0, part2.1));
}
