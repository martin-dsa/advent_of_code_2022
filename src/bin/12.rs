use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

#[derive(Debug, Clone)]
struct Cell {
    val: u8,
    visited: bool,
}

#[derive(Debug, Clone)]
struct Board {
    field: Vec<Vec<Cell>>,
}
impl Board {
    fn from(s: &str) -> Self {
        let field = s
            .split('\n')
            .map(|s| {
                s.bytes()
                    .map(|val| Cell {
                        val,
                        visited: false,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { field }
    }

    fn get(&mut self, pos: &Pos) -> Option<&mut Cell> {
        self.field.get_mut(pos.0 as usize)?.get_mut(pos.1 as usize)
    }

    fn get_end_pos(&mut self) -> Option<Pos> {
        for (x, row) in self.field.iter_mut().enumerate() {
            for (y, col) in row.iter_mut().enumerate() {
                if col.val == b'E' {
                    col.val = b'z';
                    return Some(Pos(x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        }
        None
    }
}

const DIRS: [Pos; 4] = [Pos(1, 0), Pos(-1, 0), Pos(0, 1), Pos(0, -1)];

fn check_adj<'a>(
    map: &'a mut Board,
    pos: &'a Pos,
    end_symbols: &'a [u8],
) -> impl Iterator<Item = Pos> + 'a {
    let cell = map.get(pos).unwrap();
    cell.visited = true;

    let cur_val = cell.val;

    DIRS.iter()
        .map(|p| Pos(pos.0 + p.0, pos.1 + p.1))
        .filter(move |pos| match map.get(pos) {
            Some(Cell {
                val: next_val,
                visited,
            }) => {
                !*visited
                    && (!end_symbols.contains(next_val) && *next_val >= cur_val - 1
                        || (end_symbols.contains(next_val) && (cur_val == b'a' || cur_val == b'b')))
            }
            None => false,
        })
}

fn solve(input: &str, end_symbols: Vec<u8>) -> Option<u32> {
    let mut board = Board::from(input);

    let start_idx = board.get_end_pos().unwrap();
    let mut cur_idx = vec![start_idx];

    let mut steps_count: u32 = 0;

    while !cur_idx
        .iter()
        .map(|x| board.get(x).unwrap().val as char)
        .any(|x| end_symbols.contains(&(x as u8)))
    {
        let mut new_possible_positions: HashSet<Pos> = HashSet::new();

        for idx in &cur_idx {
            for p in check_adj(&mut board, idx, &end_symbols) {
                new_possible_positions.insert(p);
            }
        }

        steps_count += 1;
        cur_idx = Vec::from_iter(new_possible_positions);
    }

    Some(steps_count)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, vec![b'S'])
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, vec![b'S', b'a'])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
