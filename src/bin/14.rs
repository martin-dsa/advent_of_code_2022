use std::{
    cmp::{max, min},
    collections::HashSet,
};

const SAND_START: (i32, i32) = (500, 0);

fn get_board(input: &str) -> (HashSet<(i32, i32)>, i32) {
    let mut board: HashSet<(i32, i32)> = HashSet::new();

    let a = input.lines().map(|l| {
        l.split(" -> ").map(|pair| {
            let temp = pair.split_once(',').unwrap();
            (
                temp.0.parse::<i32>().unwrap(),
                temp.1.parse::<i32>().unwrap(),
            )
        })
    });

    for wall in a.clone() {
        let w = wall.collect::<Vec<_>>();

        for q in w.windows(2) {
            let w1 = q[0];
            let w2 = q[1];

            if w1.0 == w2.0 {
                let sm = min(w1.1, w2.1);
                let lg = max(w1.1, w2.1);

                for y in sm..=lg {
                    board.insert((w1.0, y));
                }
            }
            if w1.1 == w2.1 {
                let sm = min(w1.0, w2.0);
                let lg = max(w1.0, w2.0);

                for x in sm..=lg {
                    board.insert((x, w1.1));
                }
            }
        }
    }

    let lower_wall = a.flat_map(|p| p.map(|x| x.1)).max().unwrap();

    (board, lower_wall)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut board, lower_wall) = get_board(input);

    let mut i = 0;
    loop {
        let mut sand = SAND_START;
        loop {
            if sand.1 >= lower_wall {
                return Some(i);
            }
            if board.get(&(sand.0, sand.1 + 1)).is_none() {
                sand.1 += 1;
                continue;
            }
            if board.get(&(sand.0 - 1, sand.1 + 1)).is_none() {
                sand.0 -= 1;
                sand.1 += 1;
                continue;
            }
            if board.get(&(sand.0 + 1, sand.1 + 1)).is_none() {
                sand.0 += 1;
                sand.1 += 1;
                continue;
            }

            board.insert(sand);
            break;
        }
        i += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut board, lower_wall) = get_board(input);

    for x in 300..700 {
        board.insert((x, lower_wall + 2));
    }

    let mut i = 0;
    loop {
        let mut sand = SAND_START;
        loop {
            if board.get(&(sand.0, sand.1 + 1)).is_none() {
                sand.1 += 1;
                continue;
            }
            if board.get(&(sand.0 - 1, sand.1 + 1)).is_none() {
                sand.0 -= 1;
                sand.1 += 1;
                continue;
            }
            if board.get(&(sand.0 + 1, sand.1 + 1)).is_none() {
                sand.0 += 1;
                sand.1 += 1;
                continue;
            }
            if sand == SAND_START {
                return Some(i + 1);
            }

            board.insert(sand);
            break;
        }
        i += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
