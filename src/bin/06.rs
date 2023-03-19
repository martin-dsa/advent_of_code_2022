use std::collections::HashSet;

const SIZE_1: usize = 4;
const SIZE_2: usize = 14;

fn solution(input: &str, size: usize) -> Option<u32> {
    let windows = input.lines().collect::<Vec<&str>>()[0]
        .as_bytes()
        .windows(size)
        .enumerate();

    for (i, w) in windows {
        let h: HashSet<&u8> = HashSet::from_iter(w);
        if h.len() == size {
            let index: u32 = u32::try_from(i + size).unwrap();
            return Some(index);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solution(input, SIZE_1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solution(input, SIZE_2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
