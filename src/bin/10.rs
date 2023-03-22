enum Instruction {
    Noop,
    Addx(isize),
}
impl Instruction {
    fn parse(s: &str) -> Self {
        let (instr, arg) = s.split_once(' ').unwrap_or(("noop", ""));
        match instr {
            "addx" => Instruction::Addx(arg.parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => panic!(""),
        }
    }
}
pub fn part_one(input: &str) -> Option<isize> {
    let mut cycles = 0;
    let mut x_value = 1;

    let mut checkpoints: Vec<isize> = vec![220, 180, 140, 100, 60, 20];
    let mut values: Vec<isize> = vec![];

    for instr in input.lines().map(Instruction::parse) {
        if checkpoints.is_empty() {
            break;
        }
        match instr {
            Instruction::Noop => {
                cycles += 1;
            }
            Instruction::Addx(v) => {
                cycles += 2;

                if cycles >= *checkpoints.last().unwrap() {
                    values.push(checkpoints.pop().unwrap() * x_value)
                }
                x_value += v;
            }
        };
    }

    Some(values.iter().sum())
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;
const TOTAL: usize = WIDTH * HEIGHT;

fn draw_px(srt: &mut String, x_value: isize, cycle: usize) {
    let sprite_idx = vec![x_value - 1, x_value, x_value + 1];
    let a = cycle % WIDTH;
    if sprite_idx.contains(&(a as isize)) {
        srt.push('#')
    } else {
        srt.push('.')
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cycles = 0;
    let mut x_value = 1;

    let mut srt = String::from("");

    for instr in input.lines().map(Instruction::parse) {
        if cycles >= TOTAL {
            break;
        }
        match instr {
            Instruction::Noop => {
                draw_px(&mut srt, x_value, cycles);
                cycles += 1;
            }
            Instruction::Addx(v) => {
                draw_px(&mut srt, x_value, cycles);
                cycles += 1;
                draw_px(&mut srt, x_value, cycles);
                cycles += 1;

                x_value += v;
            }
        };
    }

    for i in 0..HEIGHT {
        let s_ind: usize = i * WIDTH;
        let s = &srt[s_ind..(s_ind + WIDTH)];
        println!("{s}");
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
