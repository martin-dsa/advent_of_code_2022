// TODO: refactor

use std::collections::HashSet;

enum Dir {
    Up,
    Left,
    Right,
    Down,

    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

struct Move {
    direction: Dir,
    distance: u32,
}

impl Move {
    fn parse(s: &str) -> Self {
        let (dir, dist) = s.split_once(' ').unwrap();
        let direction = match dir {
            "U" => Dir::Up,
            "L" => Dir::Left,
            "R" => Dir::Right,
            "D" => Dir::Down,
            _ => panic!("123"),
        };
        let distance: u32 = dist.parse().unwrap();

        Self {
            direction,
            distance,
        }
    }
}
#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn up(&mut self) {
        self.y += 1
    }
    fn left(&mut self) {
        self.x -= 1
    }
    fn right(&mut self) {
        self.x += 1
    }
    fn down(&mut self) {
        self.y -= 1
    }

    fn mov(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => self.up(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
            Dir::Down => self.down(),

            Dir::UpRight => {
                self.up();
                self.right()
            }
            Dir::UpLeft => {
                self.up();
                self.left()
            }
            Dir::DownRight => {
                self.down();
                self.right()
            }
            Dir::DownLeft => {
                self.down();
                self.left()
            }
        };
    }
}

fn should_move_knot(a: &Coord, b: &Coord) -> bool {
    (a.x - b.x).abs() > 1 || (a.y - b.y).abs() > 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = input.lines().map(Move::parse);

    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    let mut tail_positions: HashSet<Coord> = HashSet::new();
    tail_positions.insert(tail);

    for m in moves {
        for _ in 0..m.distance {
            let prev_head = head;
            head.mov(&m.direction);

            if should_move_knot(&head, &tail) {
                tail = prev_head;
                tail_positions.insert(tail);
            };
        }
    }

    Some(tail_positions.len() as u32)
}
fn get_pos(t: &Coord, h: &Coord) -> Coord {
    let mut new_coord = *t;
    let dir = match (h.x - t.x, h.y - t.y) {
        (0, y) if y > 0 => Dir::Up,
        (0, y) if y < 0 => Dir::Down,
        (x, 0) if x > 0 => Dir::Right,
        (x, 0) if x < 0 => Dir::Left,

        (x, y) if x > 0 && y > 0 => Dir::UpRight,
        (x, y) if x < 0 && y > 0 => Dir::UpLeft,
        (x, y) if x > 0 && y < 0 => Dir::DownRight,
        (x, y) if x < 0 && y < 0 => Dir::DownLeft,
        (_, _) => panic!("!23"),
    };
    new_coord.mov(&dir);

    new_coord
}
pub fn part_two(input: &str) -> Option<u32> {
    let moves = input.lines().map(Move::parse);
    const LEN: usize = 10;
    let mut rope = [Coord { x: 0, y: 0 }; LEN];

    let mut tail_positions: HashSet<Coord> = HashSet::new();
    tail_positions.insert(Coord { x: 0, y: 0 });

    for m in moves {
        for _ in 0..m.distance {
            let head = &mut rope[0];
            head.mov(&m.direction);

            for idx in 1..LEN {
                let first = rope[idx - 1];
                let second = &mut rope[idx];

                if should_move_knot(&first, second) {
                    let is_last = idx == LEN - 1;

                    let a = get_pos(second, &first);

                    second.x = a.x;
                    second.y = a.y;

                    if is_last {
                        tail_positions.insert(*second);
                    }
                };
            }
        }
    }

    Some(tail_positions.len() as u32)
}
// 5220 too high
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
