use std::collections::HashMap;

enum Cd {
    Back,
    Forward(String),
}
enum Command {
    Cd(Cd),
    Ls,
    Dir,
    File(u32),
}
impl Command {
    fn parse(inst: &str) -> Self {
        let (prefix, body) = inst.split_once(' ').unwrap();
        match prefix {
            "$" => {
                let (first, second) = body.split_once(' ').unwrap_or(("ls", ""));

                match first {
                    "cd" => match second {
                        ".." => Self::Cd(Cd::Back),
                        dir_name => Self::Cd(Cd::Forward(String::from(dir_name))),
                    },
                    "ls" => Self::Ls,
                    _ => panic!("unexpected command"),
                }
            }
            "dir" => Self::Dir,
            size => Self::File(size.parse::<u32>().unwrap()),
        }
    }
}

fn get_dirs(input: &str) -> HashMap<Vec<String>, u32> {
    let mut dirs: HashMap<Vec<String>, u32> = HashMap::new();
    let mut cur_stack: Vec<String> = vec![];

    input.lines().for_each(|line| {
        let command = Command::parse(line);
        match command {
            Command::Cd(cd) => match cd {
                Cd::Back => {
                    cur_stack.pop();
                }
                Cd::Forward(dir) => {
                    cur_stack.push(dir);
                    dirs.insert(cur_stack.to_vec(), 0);
                }
            },
            Command::Ls => (),
            Command::Dir => (),
            Command::File(size) => {
                for i in 0..cur_stack.len() {
                    let path: Vec<String> = cur_stack[0..=i].to_vec();
                    let value = *dirs.get(&path).unwrap() + size;
                    dirs.insert(path, value);
                }
            }
        }
    });
    dirs
}

const MIN_SIZE: u32 = 100000;

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = get_dirs(input);

    let sum: u32 = dirs.values().filter(|s| **s <= MIN_SIZE).sum();
    Some(sum)
}

const TOTAL_SIZE: u32 = 70000000;
const NEEDED_SIZE: u32 = 30000000;

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = get_dirs(input);

    let root_dir = vec![String::from("/")];

    let used_space = dirs.get(&root_dir).unwrap();
    let free_space = TOTAL_SIZE - used_space;
    let space_to_find = NEEDED_SIZE - free_space;

    let mut smallest_dir = u32::MAX;

    for dir_size in dirs.values() {
        let dir_size = *dir_size;
        if dir_size <= smallest_dir && dir_size >= space_to_find {
            smallest_dir = dir_size;
        }
    }

    Some(smallest_dir)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
