enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn get_outer_count(size: usize) -> usize {
    size * 4 - 4
}

fn get_row_in_dir(grid: &Vec<&str>, (x, y): (usize, usize), dir: &Dir) -> Vec<char> {
    match dir {
        Dir::Up => grid[0..x]
            .iter()
            .rev()
            .map(|row| row.chars().nth(y).unwrap())
            .collect(),
        Dir::Down => grid[x + 1..grid.len()]
            .iter()
            .map(|row| row.chars().nth(y).unwrap())
            .collect(),
        Dir::Left => grid[x][0..y].chars().rev().collect(),
        Dir::Right => grid[x][y + 1..grid.len()].chars().collect(),
    }
}

fn is_visible(grid: &Vec<&str>, i: (usize, usize)) -> bool {
    let directions = vec![Dir::Up, Dir::Left, Dir::Right, Dir::Down];
    let cur_height = grid[i.0].chars().nth(i.1).unwrap();

    directions.iter().any(|d| {
        get_row_in_dir(grid, i, d)
            .iter()
            .all(|height| *height < cur_height)
    })
}

fn get_score(grid: &Vec<&str>, i: (usize, usize)) -> u32 {
    let dirs = vec![Dir::Down, Dir::Left, Dir::Right, Dir::Up];
    let cur_height = grid[i.0].chars().nth(i.1).unwrap();

    dirs.iter()
        .map(|d| {
            let tree_path = get_row_in_dir(grid, i, d);

            let mut count = 0;
            for height in tree_path {
                count += 1;

                if height >= cur_height {
                    break;
                }
            }

            match count {
                0 => 1,
                b => b,
            }
        })
        .product::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<_>>();

    let visible_trees_count = (1..rows.len() - 1)
        .map(|x| {
            (1..rows[x].len() - 1)
                .filter(|y| is_visible(&rows, (x, *y)))
                .collect::<Vec<_>>()
                .len()
        })
        .sum::<usize>()
        + get_outer_count(rows.len());
    Some(visible_trees_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.lines().collect::<Vec<_>>();
    let size_x = rows[0].len();
    let size_y = rows.len();
    let rows_ref = &rows;

    (1..size_x - 1)
        .flat_map(|x| (1..size_y - 1).map(move |y| get_score(rows_ref, (x, y))))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
