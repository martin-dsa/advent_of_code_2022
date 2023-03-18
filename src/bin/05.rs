use regex::Regex;
use std::collections::LinkedList;

struct Move(u32, u32, u32);
type Stacks = Vec<LinkedList<char>>;

fn apply_move_2(stacks: &mut Stacks, m: Move) {
    let Move(how_many, from, to) = m;
    let q = stacks.get_mut(from as usize - 1).unwrap();
    let mut s = q.split_off(q.len() - how_many as usize);
    stacks.get_mut(to as usize - 1).unwrap().append(&mut s);
}

fn apply_move(stacks: &mut Stacks, m: Move) {
    let Move(how_many, from, to) = m;
    for _ in 0..how_many {
        let q = stacks
            .get_mut(from as usize - 1)
            .unwrap()
            .pop_back()
            .unwrap();

        let to = stacks.get_mut(to as usize - 1).unwrap();
        to.push_back(q);
    }
}

fn get_init_state(input: &str) -> Stacks {
    let lines = input.lines().collect::<Vec<_>>();
    let (_last, grid) = lines.split_last().unwrap();

    let rows = grid
        .iter()
        .map(|&s| {
            s.as_bytes()
                .chunks(4)
                .map(|c| *c.iter().skip(1).take(1).next().unwrap() as char)
        })
        .collect::<Vec<_>>();

    let mut stacks: Vec<LinkedList<char>> = vec![];

    for _ in 0..rows[0].len() {
        let a = LinkedList::<char>::new();
        stacks.push(a);
    }

    for c in rows {
        for (i, q) in c.enumerate() {
            if ' ' != q {
                stacks[i].push_front(q)
            }
        }
    }
    stacks
}

fn get_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    re.captures_iter(input)
        .map(|captures| {
            Move(
                captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<String> {
    let (state, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = get_init_state(state);
    let moves = get_moves(moves);
    for m in moves {
        apply_move(&mut stacks, m);
    }

    Some(stacks.iter().map(|s| s.back().unwrap()).collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let (state, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = get_init_state(state);
    let moves = get_moves(moves);
    for m in moves {
        apply_move_2(&mut stacks, m);
    }

    Some(stacks.iter().map(|s| s.back().unwrap()).collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
