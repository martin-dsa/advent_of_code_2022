pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            b.chars()
                .filter(|x| a.contains(*x))
                .map(|i| i.to_digit(36).unwrap() - 9 + if i.is_uppercase() { 26 } else { 0 })
                .next()
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    lines[..]
        .chunks(3)
        .map(|ar| match ar {
            [a, b, c] => b
                .chars()
                .filter(|x| a.contains(*x) && c.contains(*x))
                .map(|i| i.to_digit(36).unwrap() - 9 + if i.is_uppercase() { 26 } else { 0 })
                .next(),
            _ => panic!("unexpected input"),
        })
        .sum()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
