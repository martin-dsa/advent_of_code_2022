use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}
#[derive(Debug, Clone)]
struct Operation {
    op: Op,
    val: Option<i128>,
}
#[derive(Debug, Clone)]
struct Test {
    value: i128,
    true_case: usize,
    false_case: usize,
}
#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    test: Test,
}
impl Monkey {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref SI: Regex = Regex::new(r"Starting items: (.*)").unwrap();
            static ref OP: Regex = Regex::new(r"Operation: new = old (\*|\+) (\d+|old)").unwrap();
            static ref TEST: Regex = Regex::new(
                r"Test: divisible by (\d*)
    If true: throw to monkey (\d*)
    If false: throw to monkey (\d*)"
            )
            .unwrap();
        }

        let lines: Vec<&str> = s.split('\n').collect();

        let _title = lines[0];

        let starting_items = lines[1];
        let starting_items = SI
            .captures(starting_items)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse::<i128>().unwrap())
            .collect();

        let operation = lines[2];

        let operation = OP
            .captures(operation)
            .map(|captures| {
                (
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str().parse::<i128>(),
                )
            })
            .unwrap();

        let operation = match operation {
            ("+", v) => Operation {
                op: Op::Add,
                val: v.ok(),
            },
            ("*", v) => Operation {
                op: Op::Mul,
                val: v.ok(),
            },
            (_, _) => panic!(""),
        };

        let test = &lines[3..=5].join("\n");
        let (value, true_case, false_case) = TEST
            .captures(test)
            .map(|captures| {
                (
                    captures.get(1).unwrap().as_str().parse::<i128>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                )
            })
            .unwrap();
        let test = Test {
            value,
            true_case,
            false_case,
        };

        Self {
            operation,
            items: starting_items,
            test,
        }
    }

    fn test(&self, val: i128) -> usize {
        let Test {
            value,
            true_case,
            false_case,
        } = self.test;

        let idx = if val % value == 0 {
            true_case
        } else {
            false_case
        };

        idx
    }
}

const TOTAL_ROUNDS_1: usize = 20;

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..TOTAL_ROUNDS_1 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();

            for item in monkey.items.iter() {
                let score = match monkey.operation.op {
                    Op::Add => item + monkey.operation.val.unwrap_or(*item),
                    Op::Mul => item * monkey.operation.val.unwrap_or(*item),
                };
                let score = score / 3;

                let new_monkey_idx = monkey.test(score);
                monkeys[new_monkey_idx].items.push(score);

                inspections[i] += 1;
            }
            monkeys[i].items = vec![];
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    Some(inspections[0] * inspections[1])
}

// const TOTAL_ROUNDS_2: usize = 10;
const TOTAL_ROUNDS_2: usize = 10_000;
pub fn part_two(input: &str) -> Option<u128> {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    let mut inspections: Vec<u128> = vec![0; monkeys.len()];

    let lcm = monkeys.iter().map(|x| x.test.value).product::<i128>();

    for _round in 0..TOTAL_ROUNDS_2 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();

            for item in monkey.items.iter() {
                let score = match monkey.operation.op {
                    Op::Add => item + monkey.operation.val.unwrap_or(*item),
                    Op::Mul => item * monkey.operation.val.unwrap_or(*item),
                };
                let score = score % lcm;

                let new_monkey_idx = monkey.test(score);
                monkeys[new_monkey_idx].items.push(score);

                inspections[i] += 1;
            }
            monkeys[i].items = vec![];
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));

    Some(inspections[0] * inspections[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
