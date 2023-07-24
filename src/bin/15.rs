use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r".*x=(-?\d+), y=(-?\d+).*").unwrap();
        }
        Ok(RE
            .captures(s)
            .map(|captures| Point {
                x: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                y: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            })
            .unwrap())
    }
}

impl Point {
    fn mh_dist(&self, other: &Self) -> u32 {
        (self.x.abs_diff(other.x)) + (self.y.abs_diff(other.y))
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Sensor at (x=-?\d+, y=-?\d+): closest beacon is at x=(-?\d+, y=-?\d+)")
            .unwrap();
}

fn get_data(input: &str) -> impl Iterator<Item = (Point, Point, u32)> + '_ {
    input.lines().map(|line| {
        let (sensor_str, beacon_str) = line.split_once(':').unwrap();

        let sensor: Point = sensor_str.parse().unwrap();
        let beacon: Point = beacon_str.parse().unwrap();
        let mh_dist = sensor.mh_dist(&beacon);

        (sensor, beacon, mh_dist)
    })
}
pub fn part_one(input: &str) -> Option<u32> {
    let y: i32 = if cfg!(test) { 10 } else { 2000000 };
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut res: HashSet<Point> = HashSet::new();

    let data = get_data(input);

    for (sensor, beacon, mh_dist) in data {
        beacons.insert(beacon);
        let offset = mh_dist as i32 - sensor.y.abs_diff(y) as i32;

        for x in (sensor.x - offset)..=(sensor.x + offset) {
            res.insert(Point { x, y });
        }
    }

    let res = res.difference(&beacons).filter(|p| p.y == y).count() as u32;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u128> {
    let data = get_data(input);
    let mut pos_lines: Vec<i32> = vec![];
    let mut neg_lines: Vec<i32> = vec![];

    for (sensor, _, mh_dist) in data {
        let mh_dist = mh_dist as i32;
        pos_lines.push(sensor.x - sensor.y - mh_dist);
        pos_lines.push(sensor.x - sensor.y + mh_dist);
        neg_lines.push(sensor.x + sensor.y - mh_dist);
        neg_lines.push(sensor.x + sensor.y + mh_dist);
    }

    let mut pos: i64 = 0;
    let mut neg: i64 = 0;

    for i in 0..pos_lines.len() {
        for j in (i + 1)..pos_lines.len() {
            let a = pos_lines[i];
            let b = pos_lines[j];

            if a.abs_diff(b) == 2 {
                pos = a.min(b) as i64 + 1;
            }

            let a = neg_lines[i];
            let b = neg_lines[j];

            if a.abs_diff(b) == 2 {
                neg = a.min(b) as i64 + 1;
            }
        }
    }

    let x = ((pos + neg) / 2) as u128;
    let y = ((neg - pos) / 2) as u128;

    let res = x * 4000000 + y;

    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(52000002));
    }
}
