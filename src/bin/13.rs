use core::fmt;
use std::cmp::Ordering::{self, Equal, Greater, Less};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    N(usize),
    A(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::N(n1), Packet::N(n2)) => n1.cmp(n2),
            (Packet::A(arr1), Packet::A(arr2)) => cmp_arr(arr1, arr2),
            (Packet::N(n), Packet::A(arr)) => cmp_arr(&[Packet::N(*n)], arr),
            (Packet::A(arr), Packet::N(n)) => cmp_arr(arr, &[Packet::N(*n)]),
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::N(n) => write!(f, "({})", n),
            Packet::A(arr) => write!(f, "({:?})", arr),
        }
    }
}

fn cmp_arr(a: &[Packet], b: &[Packet]) -> Ordering {
    let mut cursor: Ordering = Equal;
    let mut i = 0;

    while cursor == Equal {
        let cur_a = a.get(i);
        let cur_b = b.get(i);

        cursor = match (cur_a, cur_b) {
            (None, Some(_)) => Less,
            (Some(_), None) => Greater,
            (None, None) => return Equal,
            (Some(a), Some(b)) => a.cmp(b),
        };

        i += 1;
    }

    cursor
}

pub fn part_one(input: &str) -> Option<usize> {
    let a: usize = input
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap())
        .map(|(a, b)| {
            let a: Packet = serde_json::from_str(a).unwrap();
            let b: Packet = serde_json::from_str(b).unwrap();
            (a, b)
        })
        .map(|(a, b)| a.cmp(&b))
        .enumerate()
        .filter(|(_, b)| *b == Less)
        .map(|(i, _)| i + 1)
        .sum();
    Some(a)
}

pub fn part_two(input: &str) -> Option<usize> {
    let div_packet_1 = Packet::A(vec![Packet::A(vec![Packet::N(2)])]);
    let div_packet_2 = Packet::A(vec![Packet::A(vec![Packet::N(6)])]);

    let mut packets: Vec<Packet> = input
        .split('\n')
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();

    packets.push(div_packet_1.clone());
    packets.push(div_packet_2.clone());

    packets.sort();

    let idx_1 = packets.iter().position(|p| p.eq(&div_packet_1)).unwrap() + 1;
    let idx_2 = packets.iter().position(|p| p.eq(&div_packet_2)).unwrap() + 1;

    Some(idx_1 * idx_2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
