use std::collections::HashMap;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|n| n.chars().map(|f| f.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn run(input: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            sum += find_risk_level(input, x, y);
        }
    }
    sum
}

fn find_risk_level(input: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let value = input[y][x];
    if y > 0 {
        let above = input[y - 1][x];
        if above <= value {
            return 0;
        }
    }
    if x > 0 {
        let left = input[y][x - 1];
        if left <= value {
            return 0;
        }
    }
    if y < input.len() - 1 {
        let below = input[y + 1][x];
        if below <= value {
            return 0;
        }
    }
    if x < input[y].len() - 1 {
        let right = input[y][x + 1];
        if right <= value {
            return 0;
        }
    }

    value + 1
}

#[aoc(day9, part2)]
pub fn run_p2(input: &[Vec<u32>]) -> u32 {
    let mut basins = Basins::new(input);
    let mut basin_sizes: HashMap<Coord, u32> = HashMap::new();
    // for each coordinate find its low-point and +1 the corresponding value
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if let Some(low_point) = basins.find_low_point(x, y) {
                *basin_sizes.entry(low_point).or_insert(0) += 1;
            }
        }
    }
    let mut basin_sizes = basins.basin_representatives.iter().map(|c| basin_sizes[c]).collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}

impl Coord {
    fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Coord {
        Coord { x, y, max_x, max_y }
    }

    fn neighbours(&self) -> Vec<Coord> {
        let mut result = Vec::new();

        if self.y > 0 {
            result.push(Coord { x: self.x, y: self.y - 1, max_x: self.max_x, max_y: self.max_y });
        }
        if self.x + 1 < self.max_x {
            result.push(Coord { x: self.x + 1, y: self.y, max_x: self.max_x, max_y: self.max_y });
        }
        if self.y + 1 < self.max_y {
            result.push(Coord { x: self.x, y: self.y + 1, max_x: self.max_x, max_y: self.max_y });
        }
        if self.x > 0 {
            result.push(Coord { x: self.x - 1, y: self.y, max_x: self.max_x, max_y: self.max_y });
        }

        result
    }
}

struct Basins<'a> {
    grid: &'a [Vec<u32>],
    low_points: Vec<Vec<Option<Coord>>>,
    basin_representatives: Vec<Coord>,
}

impl<'a> Basins<'a> {
    fn new(input: &[Vec<u32>]) -> Basins {
        Basins {
            grid: input,
            low_points: vec![vec![None; input[0].len()]; input.len()],
            basin_representatives: Vec::new(),
        }
    }

    fn find_low_point(&mut self, x: usize, y: usize) -> Option<Coord> {
        let value = self.grid[y][x];
        if value == 9 {
            return None;
        }
        if let Some(low_point) = self.low_points[y][x] {
            return Some(low_point);
        }

        let coord = self.coord_at(x, y);
        let neighbours = coord.neighbours().into_iter().map(|c| (self.grid[c.y][c.x], c));
        let (min_val, min_coord) = neighbours.min_by_key(|t| t.0).expect("every point has at least two neighbours");
        let low_point = if min_val < value {
            self.find_low_point(min_coord.x as usize, min_coord.y as usize)
        } else {
            let result = coord;
            self.basin_representatives.push(result);
            Some(result)
        };
        self.low_points[y][x] = low_point;
        low_point
    }

    fn coord_at(&self, x: usize, y: usize) -> Coord {
        Coord::new(x, y, self.grid[0].len(), self.grid.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day9.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 448);
    }

    #[test]
    fn input_known_answer_p2() {
        let result = run_p2(&input_generator(INPUT));

        assert_eq!(result, 1417248);
    }

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn example() {
        let parsed = input_generator(EXAMPLE);
        let actual = run(&parsed);

        assert_eq!(actual, 15);
    }

    #[test]
    fn example_p2() {
        let parsed = input_generator(EXAMPLE);
        let actual = run_p2(&parsed);

        assert_eq!(actual, 1134);
    }

    const EXAMPLE2: &str = "2199943210
3987894921
2856789892
8767896789
7899965678";

    #[test]
    fn example2() {
        let parsed = input_generator(EXAMPLE2);
        let actual = run(&parsed);

        assert_eq!(actual, 26);
    }
}
