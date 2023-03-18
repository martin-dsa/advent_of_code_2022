struct Range(u32, u32);
impl Range {
    fn from_str((s1, s2): (&str, &str)) -> Self {
        Range(s1.parse().unwrap(), s2.parse().unwrap())
    }

    fn full_includes(&self, Range(s2, e2): &Range) -> bool {
        let Range(s1, e1) = self;
        s1 >= s2 && e1 <= e2
    }

    fn part_includes(&self, Range(s2, e2): &Range) -> bool {
        let Range(s1, e1) = self;
        (s1 >= s2 || e1 >= s2) && (s1 <= e2 || e1 <= e2)
    }

    fn sym_full_includes(&self, r2: &Range) -> bool {
        Self::full_includes(self, r2) || Self::full_includes(r2, self)
    }
    fn sym_part_includes(&self, r2: &Range) -> bool {
        Self::part_includes(self, r2) || Self::part_includes(r2, self)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .filter_map(|(a, b)| {
            let r1 = Range::from_str(a.split_once('-').unwrap());
            let r2 = Range::from_str(b.split_once('-').unwrap());
            r1.sym_full_includes(&r2).then_some(1u32)
        })
        .reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .filter_map(|(a, b)| {
            let r1 = Range::from_str(a.split_once('-').unwrap());
            let r2 = Range::from_str(b.split_once('-').unwrap());
            r1.sym_part_includes(&r2).then_some(1u32)
        })
        .reduce(|a, b| a + b)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
