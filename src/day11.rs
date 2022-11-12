use std::collections::{HashMap, HashSet};

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

pub struct OctoField {
    entries: Vec<u8>,
    flashes: u64,
}

impl OctoField {
    fn new(entries: Vec<u8>) -> Self {
        OctoField {entries, flashes: 0}
    }
    
    fn step(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry += 1;
        }
        
        let mut prev_len = 1;
        let mut flashed: HashSet<usize> = HashSet::new();
        while flashed.len() != prev_len {
            prev_len = flashed.len();
            self.handle_flashes(&mut flashed);
        }
        self.flashes += flashed.len() as u64;
        for idx in flashed {
            self.entries[idx] = 0;
        }
    }
    
    fn handle_flashes(&mut self, flashed: &mut HashSet<usize>) {
        let mut to_up = Vec::new();
        for (index, entry) in self.entries.iter_mut().enumerate() {
            if *entry >= 9 {
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
        if index % 10 > 0 {
            result.push(index - 1);
        }
        if index % 10 < 9 {
            result.push(index + 1);
        }
        if index > 9 {
            result.push(index - 10);
        }
        if index < 90 {
            result.push(index + 10);
        }
        result        
    }
}

#[aoc(day11, part1)]
pub fn run(input: &OctoField) -> u64 {
    todo!()
}

// #[aoc(day11, part2)]
// pub fn run_p2(input: &[u32]) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day11.txt");

    #[test]
    fn input_known_answer() {
        let result = run(&input_generator(INPUT));

        assert_eq!(result, 0);
    }

    // #[test]
    // fn input_known_answer_p2() {
    //     let result = run_p2(&input_generator(INPUT));
    //
    //     assert_eq!(result, 0);
    // }

    const EXAMPLE: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn example() {
        let actual = run(&OctoField::new(Vec::from(EXAMPLE)));

        assert_eq!(actual, 0);
    }
}
