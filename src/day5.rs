use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<(Coord, Coord)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Coord, Coord) {
    line.split_once(" -> ")
        .map(|(start, end)| (Coord::from(start), Coord::from(end)))
        .unwrap()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Coord {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Coord {
    fn from(tuple: (u16, u16)) -> Self {
        Coord {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<&str> for Coord {
    fn from(str: &str) -> Self {
        let x_y = str
            .split_once(",")
            .map(|(x, y)| {
                (
                    x.parse::<u16>().expect(&*format!("x: {:?}", x)),
                    y.parse::<u16>().expect(&*format!("x: {:?}", x)),
                )
            })
            .unwrap();
        Coord::from(x_y)
    }
}

#[aoc(day5, part1)]
pub fn run(input: &[(Coord, Coord)]) -> usize {
    let horiz_and_vertical = input
        .iter()
        .filter(|(start, end)| start.x == end.x || start.y == end.y);
    let filled_map = horiz_and_vertical
        .flat_map(|(start, end)| Line::new(*start, *end))
        .into_iter()
        .fold(HashMap::<Coord, u8>::new(), |mut map, point| {
            map.entry(point)
                .and_modify(|count| *count += 1u8)
                .or_insert(1);
            map
        });

    filled_map.into_iter().filter(|kvp| kvp.1 >= 2).count()
}

#[aoc(day5, part2)]
pub fn run_incl_diag(input: &[(Coord, Coord)]) -> usize {
    let filled_map = input
        .iter()
        .flat_map(|(start, end)| Line::new(*start, *end))
        .into_iter()
        .fold(HashMap::<Coord, u8>::new(), |mut map, point| {
            map.entry(point)
                .and_modify(|count| *count += 1u8)
                .or_insert(1);
            map
        });

    filled_map.into_iter().filter(|kvp| kvp.1 >= 2).count()
}

struct Line {
    current_point: Coord,
    end_point: Coord,
    started: bool,
}

impl Line {
    fn new(start_point: Coord, end_point: Coord) -> Line {
        Line {
            current_point: start_point,
            end_point,
            started: false,
        }
    }
}

impl Iterator for Line {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.current_point);
        }
        if self.current_point == self.end_point {
            return None;
        }

        match self.current_point.x.cmp(&self.end_point.x) {
            Ordering::Greater => self.current_point.x -= 1,
            Ordering::Less => self.current_point.x += 1,
            Ordering::Equal => (),
        }
        match self.current_point.y.cmp(&self.end_point.y) {
            Ordering::Greater => self.current_point.y -= 1,
            Ordering::Less => self.current_point.y += 1,
            Ordering::Equal => (),
        }

        Some(self.current_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day5.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 8622);
    }

    #[test]
    fn input_known_answer_p2() {
        let result = run_incl_diag(&input_generator(INPUT));

        assert_eq!(result, 22037);
    }

    const EXAMPLE: &str = include_str!("../input/2021/day5_example.txt");

    #[test]
    fn example_has_5_overlapping_points() {
        let count = run(&input_generator(EXAMPLE));

        assert_eq!(count, 5);
    }

    #[test]
    fn example_has_12_overlapping_points_when_including_diagonals() {
        let count = run_incl_diag(&input_generator(EXAMPLE));

        assert_eq!(count, 12);
    }
}
