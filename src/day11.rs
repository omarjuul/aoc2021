use std::collections::HashSet;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> OctoField {
    let entries: Vec<_> = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("should be a number") as u8)
        })
        .collect();
    OctoField::new(entries)
}

#[derive(Clone)]
pub struct OctoField {
    entries: Vec<u8>,
    flashes: u64,
}

impl OctoField {
    fn new(entries: Vec<u8>) -> Self {
        OctoField {
            entries,
            flashes: 0,
        }
    }

    fn step(&mut self) -> u64 {
        for entry in self.entries.iter_mut() {
            *entry += 1;
        }

        let mut prev_len = 101;
        let mut flashed: HashSet<usize> = HashSet::new();
        while flashed.len() != prev_len {
            prev_len = flashed.len();
            self.handle_flashes(&mut flashed);
            // println!("{} octo's flashed", flashed.len());
        }
        let num_flashed = flashed.len() as u64;
        for idx in flashed {
            self.entries[idx] = 0;
        }
        self.flashes += num_flashed;
        // self.print();
        num_flashed
    }

    fn handle_flashes(&mut self, flashed: &mut HashSet<usize>) {
        let mut to_up = Vec::new();
        for (index, entry) in self.entries.iter_mut().enumerate() {
            if *entry > 9 {
                let flashes = flashed.insert(index);
                if flashes {
                    to_up.push(index);
                }
            }
        }
        for index in to_up {
            self.up_neighbours(index);
        }
    }

    fn up_neighbours(&mut self, index: usize) {
        let neighbours = self.neighbours_of(index);
        for neighbour in neighbours {
            self.entries[neighbour] += 1;
        }
    }

    fn neighbours_of(&self, index: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let can_go_left = index % 10 > 0;
        let can_go_right = index % 10 < 9;
        let can_go_up = index > 9;
        let can_go_down = index < 90;
        if can_go_up {
            result.push(index - 10);
            if can_go_left {
                result.push(index - 11);
            }
            if can_go_right {
                result.push(index - 9)
            }
        }
        if can_go_left {
            result.push(index - 1);
        }
        if can_go_right {
            result.push(index + 1);
        }
        if can_go_down {
            result.push(index + 10);
            if can_go_left {
                result.push(index + 9);
            }
            if can_go_right {
                result.push(index + 11)
            }
        }
        result
    }

    fn print(&self) {
        for line in self.entries.chunks(10) {
            println!("{:?}", line);
        }
        println!();
    }
}

#[aoc(day11, part1)]
pub fn run(input: &OctoField) -> u64 {
    let mut input = input.clone(); // ugly but it works
    for _ in 0..100 {
        input.step();
    }
    input.flashes
}

#[aoc(day11, part2)]
pub fn run_p2(input: &OctoField) -> u64 {
    let mut input = input.clone(); // ugly but it works
    for i in 1.. {
        let num_flashes = input.step();
        if num_flashes >= 100 {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day11.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 1739);
    }

    #[test]
    fn input_known_answer_p2() {
        let result = run_p2(&input_generator(INPUT));

        assert_eq!(result, 324);
    }

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn example() {
        let actual = run(&input_generator(EXAMPLE));

        assert_eq!(actual, 1656);
    }

    #[test]
    fn example_p2() {
        let actual = run_p2(&input_generator(EXAMPLE));

        assert_eq!(actual, 195);
    }
}
