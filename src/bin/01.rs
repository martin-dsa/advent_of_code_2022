use advent_of_code::helpers::parse_calories;

pub fn part_one(input: &str) -> Option<u32> {
    let max_calories = parse_calories(input).max_by(|v1, v2| v1.cmp(v2));

    if let Some(0) = max_calories {
        return None;
    }

    max_calories
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = parse_calories(input).collect::<Vec<_>>();

    calories.sort_by(|a, b| b.cmp(a));

    calories.get(0..3).map(|c| c.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
